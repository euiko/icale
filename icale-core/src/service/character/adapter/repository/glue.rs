use crate::service::character::domain::*;
use crate::service::character::port::repository::*;

use std::fmt::Write;
use std::ops::Deref;
use std::option::Option;
use std::sync::Arc;

use crate::core::glue;
use async_trait::async_trait;
use futures::executor::block_on;
use gluesql::{execute, parse, Payload, SledStorage, Query};
use serde::{Deserialize, Serialize};
use sql_builder::prelude::*;
use sql_builder::SqlBuilder;

const TableName: &str = "character";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_uri")]
    pub uri: String,
}

fn default_enabled() -> bool {
    true
}
fn default_uri() -> String {
    String::from("glue.db")
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enabled: default_enabled(),
            uri: default_uri(),
        }
    }
}

fn build_queries(sql_builders : Vec<&mut SqlBuilder>) -> Result<Vec<gluesql::Query>, ErrorKind> {
    let queries: Result<Vec<String>, ErrorKind> = sql_builders
        .iter()
        .map(|builder: &&mut SqlBuilder| {
            builder
                .sql()
                .map_err(|_| ErrorKind::Driver("failed to build query".to_string()))
        })
        .collect();
    let sql_string = queries?.iter().fold(String::new(), |mut str, query| {
        writeln!(&mut str, "{};", query);
        str
    });
    println!("sql string {}", sql_string);
    Ok(parse(&sql_string).map_err(|e| ErrorKind::Driver(format!("parse query failed: {}", e)))?)
}

fn build_query(sql_builder: &mut SqlBuilder) -> Result<gluesql::Query, ErrorKind> {
    let mut queries = build_queries(vec![sql_builder])?;
    if queries.len() != 1 {
        return Err(ErrorKind::Driver(format!(
            "query must have only one statement, got {} statements",
            queries.len()
        )));
    }

    queries.pop().ok_or(ErrorKind::Driver(
        "cannot build query, no sufficient query".to_string(),
    ))
}

// TotalRecords are only meant to be used internally for this store implementation
// to help deserialize total_records data
#[derive(Debug, Serialize, Deserialize)]
struct TotalRecords {
    total_records: i64,
}

// TotalPages are only meant to be used internally for this store implementation
// to help deserialize total_pages data
#[derive(Debug, Serialize, Deserialize)]
struct TotalPages {
    total_pages: Option<i64>,
}

// Model represent underlying actual data structure in the glue's store implementation
#[derive(Debug, Serialize, Deserialize)]
struct Model {
    pub id: String,
    pub profile_id: String,
    pub profile_name: String,
    pub profile_gender: i64,
    pub appearance_skin: i64,
}

impl Model {
    fn from_domain(character: Character) -> Model {
        Model {
            id: character.get_id().clone(), // for id will be cloned instead
            profile_id: character.profile.id,
            profile_name: character.profile.name,
            profile_gender: character.profile.gender as i64,
            appearance_skin: character.appearance.skin as i64,
        }
    }
    fn into_domain(self) -> Character {
        Character::new_with_id(
            Profile {
                id: self.profile_id,
                name: self.profile_name,
                gender: match self.profile_gender {
                    i if i == Gender::Female as i64 => Gender::Female,
                    i if i == Gender::Male as i64 => Gender::Male,
                    _ => Gender::Unknown,
                },
            },
            Appearance {
                skin: match self.appearance_skin {
                    i if i == Skin::Black as i64 => Skin::Black,
                    i if i == Skin::White as i64 => Skin::White,
                    i if i == Skin::Yellow as i64 => Skin::Yellow,
                    _ => Skin::Unknown,
                },
            },
            self.id,
        )
    }
}

pub struct GlueRepository {
    config: Config,
    storage: Option<Arc<SledStorage>>,
}

impl GlueRepository {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            storage: Option::None,
        }
    }

    pub fn get_storage(&self) -> Result<SledStorage, ErrorKind> {
        Ok(self
            .storage
            .as_ref()
            .ok_or(ErrorKind::NotEnabled)?
            .clone()
            .deref()
            .clone())
    }
}

#[async_trait(?Send)]
impl Repository for GlueRepository {
    fn init(&mut self, cfg: &config::Config) -> Option<ErrorKind> {
        match cfg.get::<Config>("db.glue") {
            Ok(val) => self.config = val,
            Err(e) => {
                return Some(ErrorKind::InitializationFailed(format!(
                    "failed to obtain configuration : {}",
                    e
                )))
            }
        };

        if !self.config.enabled {
            return None;
        }

        let storage = match SledStorage::new(self.config.uri.as_str()) {
            Ok(val) => val,
            Err(e) => {
                return Some(ErrorKind::InitializationFailed(format!(
                    "failed to create new storage : {}",
                    e
                )))
            }
        };
        self.storage = Some(Arc::new(storage));

        let storage = self.get_storage().unwrap();
        parse(
            "
            DROP TABLE IF EXISTS character;
            CREATE TABLE character (
                id TEXT UNIQUE,
                profile_id TEXT UNIQUE,
                profile_name TEXT,
                profile_gender INTEGER,
                appearance_skin INTEGER
            );
        ",
        )
        .unwrap()
        .iter()
        .fold(storage, |storage, sql| {
            let (storage, _) = block_on(execute(storage, sql)).unwrap();
            storage
        });

        None
    }

    fn close(&mut self) -> Option<ErrorKind> {
        None
    }

    async fn create(&mut self, character: Character) -> Result<Character, ErrorKind> {
        let query = build_query(
            SqlBuilder::insert_into(TableName)
                .fields(&[
                    "id",
                    "profile_id",
                    "profile_name",
                    "profile_gender",
                    "appearance_skin",
                ])
                .values(&[
                    &quote(character.get_id()),
                    &quote(&character.profile.id),
                    &quote(&character.profile.name),
                    &(character.profile.gender.clone() as i32).to_string(),
                    &(character.appearance.skin.clone() as i32).to_string(),
                ]),
        )?;
        let storage = self.get_storage()?;
        match execute(storage, &query).await {
            Ok((_storage, _)) => Ok(character),
            Err((_storage, err)) => {
                Err(ErrorKind::Driver(format!("execute query failed: {}", err)))
            }
        }
    }
    async fn get_by_profile_id(&mut self, profile_id: &str) -> Result<Character, ErrorKind> {
        let query = build_query(
            SqlBuilder::select_from(TableName)
                .field("*")
                .and_where_eq("profile_id", &quote(profile_id)),
        )?;
        let storage = self.get_storage()?;
        match execute(storage, &query).await {
            Ok((_storage, payload)) => {
                Ok(glue::deserialize_one::<Model>(payload)
                .or_else(|err| Err(ErrorKind::Driver(format!("{}", err))))?
                .into_domain())
            },
            Err((_storage, err)) => {
                Err(ErrorKind::Driver(format!("execute query failed: {}", err)))
            }
        }
    }
    async fn find(&mut self, params: FindParams) -> Result<FindResult, ErrorKind> {
        // filter_builder help to build filter query
        let filter_builder = |sql: &mut SqlBuilder| {
            let keyword = params.keyword.clone().unwrap_or_default();
            if keyword != "" {
                sql.and_where_like_any(
                    "LOWER(profile_name) || ' ' || LOWER(profile_id)",
                    keyword.to_lowercase(),
                );
            }
            if let Some(profile_id) = params.profile_id.clone() {
                sql.and_where_eq("profile_id", &quote(profile_id));
            }

            if let Some(profile_name) = params.profile_name.clone() {
                sql.and_where_eq("profile_name", &quote(profile_name));
            }
        };

        //  data sql builder
        let mut select_sql = SqlBuilder::select_from(TableName);
        select_sql.field("*").offset(params.skip);
        {
            if let Some(limit) = params.limit {
                select_sql.limit(limit);
            }
        }
        filter_builder(&mut select_sql);

        // total_records sql builder
        let mut total_records_sql = SqlBuilder::select_from(TableName);
        total_records_sql
            .count_as("id", "total_records");
        filter_builder(&mut total_records_sql);

        let queries = build_queries(vec![
            &mut select_sql,
            &mut total_records_sql,
        ])?;

        let mut result = FindResult {
            records: Vec::new(),
            total_pages: Some(0),
            total_records: 0,
        };

        type CollectorFn = fn(&FindParams, FindResult, Payload) -> Result<FindResult, ErrorKind>;
        const collectors: [CollectorFn; 3] = [
            |_params, mut result, payload| -> Result<FindResult, ErrorKind> {
                let data = glue::deserialize::<Model>(payload)
                    .or_else(|err| {
                        Err(ErrorKind::Driver(format!("failed to get total_pages data")))
                    })?
                    .into_iter()
                    .map(|d| d.into_domain())
                    .collect();
                result.records = data;
                Ok(result)
            },
            |_params, mut result, payload| -> Result<FindResult, ErrorKind> {
                let data = glue::deserialize_one::<TotalRecords>(payload).or_else(|_err| {
                    Err(ErrorKind::Driver(format!(
                        "failed to get total_records data"
                    )))
                })?;
                result.total_records = data.total_records;
                Ok(result)
            },
            |params, mut result, _payload| -> Result<FindResult, ErrorKind> {
                if let Some(limit) = params.limit {
                    result.total_pages =
                        Some((result.total_records as f64 / limit as f64).ceil() as i64)
                }
                Ok(result)
            },
        ];
        let mut storage = self.get_storage()?;

        let mut queries: Vec<Option<Query>> = queries.into_iter().map(|q| Some(q)).collect();
        queries.push(None);

        for (f, query) in collectors.iter().zip(queries.iter()).into_iter() {
            if let Some(query) = query {
                match execute(storage, &query).await {
                    Ok((_storage, payload)) => {
                        storage = _storage;
                        result = f(&params, result, payload)?;
                    }
                    Err((_storage, err)) => {
                        return Err(ErrorKind::Driver(format!("execute query failed: {}", err)));
                    }
                }
            } else {
                result = f(&params, result, Payload::Select{
                    labels: vec![],
                    rows: vec![],
                })?;
            }
        }

        Ok(result)
    }
    async fn update(&mut self, character: Character) -> Result<Character, ErrorKind> {
        todo!()
    }
    async fn patch(&mut self, character: Character) -> Result<Character, ErrorKind> {
        todo!()
    }
    async fn remove(&mut self, character: Character) -> Result<Character, ErrorKind> {
        todo!()
    }
    async fn remove_by_profile_id(&mut self, profile_id: &str) -> Result<Character, ErrorKind> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    fn new_repository() -> GlueRepository {
        let mut glueRepo = GlueRepository::new();

        let mut config = config::Config::new();
        let glueCfg: HashMap<String, String> = HashMap::new();
        config.set("db.glue", glueCfg).unwrap();

        match glueRepo.init(&config) {
            Some(err) => panic!(err),
            None => glueRepo,
        }
    }

    #[test]
    fn new() {
        let mut glueRepo = new_repository();
        glueRepo.close();
    }

    #[test]
    fn crud() {
        let mut glueRepo = new_repository();

        let mut char1 = Character::new(
            Profile {
                id: "euiko".to_string(),
                name: "Candra Kharista".to_string(),
                gender: Gender::Male,
            },
            Appearance { skin: Skin::Yellow },
        );

        char1 = block_on(glueRepo.create(char1)).unwrap();
        let get_char1 = block_on(glueRepo.get_by_profile_id(&char1.profile.id)).unwrap();
        assert_eq!(char1, get_char1);

        let find_chars = block_on(glueRepo.find(FindParams{
            keyword: None,
            profile_name: Some(char1.profile.name.to_string()),
            profile_id: None,
            skip: 0,
            limit: None,
        })).unwrap();
        println!("find result is: {:?}", find_chars);
    }
}

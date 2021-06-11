use super::*;

use std::ops::Deref;
use std::option::Option;
use std::sync::Arc;

use async_trait::async_trait;
use futures::executor::block_on;
use gluesql::{execute, parse, SledStorage};
use serde::{Deserialize, Serialize};
use sql_builder::prelude::*;
use sql_builder::SqlBuilder;
use crate::core::glue;

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

fn build_queries(sql_builder: &mut SqlBuilder) -> Result<Vec<gluesql::Query>, ErrorKind> {
    let sql_string = sql_builder
        .sql()
        .map_err(|_| ErrorKind::Driver("failed to build sql script".to_string()))?;
    Ok(parse(&sql_string).map_err(|e| ErrorKind::Driver(format!("parse query failed: {}", e)))?)
}

fn build_query(sql_builder: &mut SqlBuilder) -> Result<gluesql::Query, ErrorKind> {
    let mut queries = build_queries(sql_builder)?;
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
        Model{
            id: character.get_id().clone(), // for id will be cloned instead
            profile_id: character.profile.id,
            profile_name: character.profile.name,
            profile_gender: character.profile.gender as i64,
            appearance_skin: character.appearance.skin as i64,
        }
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
            SqlBuilder::insert_into("character")
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
            Ok((_storage, _)) => {
                Ok(character)
            }
            Err((_storage, err)) => {
                Err(ErrorKind::Driver(format!("execute query failed: {}", err)))
            }
        }
    }
    async fn get_by_profile_id(&mut self, profile_id: &str) -> Result<Character, ErrorKind> {
        let query = build_query(
            SqlBuilder::select_from("character")
                .field("*")
                .and_where_eq("profile_id", &quote(profile_id)),
        )?;
        let storage = self.get_storage()?;
        match execute(storage, &query).await {
            Ok((_storage, payload)) => glue::deserialize_one::<Character>(payload).or_else(|_| Err(ErrorKind::Driver("".to_string()))),
            Err((_storage, err)) => Err(ErrorKind::Driver(format!("execute query failed: {}", err)))
        }

    }
    async fn find(&mut self, params: FindParams) -> Result<FindResult, ErrorKind> {
        todo!()
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

        let mut char1 = Character::new(Profile{
            id: "euiko".to_string(),
            name: "Candra Kharista".to_string(),
            gender: Gender::Male,
        }, Appearance{
            skin: Skin::Yellow,
        });

        char1 = block_on(glueRepo.create(char1)).unwrap();
        block_on(glueRepo.get_by_profile_id(&char1.profile.id)).unwrap();
    }
}

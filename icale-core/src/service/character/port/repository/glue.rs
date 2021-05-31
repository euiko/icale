use super::*;
use async_trait::async_trait;
use gluesql::{parse, Glue, SledStorage};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub enabled: bool,
    pub uri: String,
}

impl Default for Config {
fn default() -> Self { 
    Config{
        enabled: true,
        uri: String::from("glue.db"),
    }
}
}

pub struct GlueRepository {
    config: Config,
    client: Option<Arc<Glue>>,
}

impl GlueRepository {
    pub fn new() -> Self {
        Self{
            config: Config::default(),
            client: Option::None,
        }
    }

    pub fn get_client(&self) -> Result<Arc<Glue>, ErrorKind> {
        Ok(self.client.as_ref().ok_or(ErrorKind::NotEnabled)?.clone())
    }
}

#[async_trait]
impl Repository for GlueRepository {
    fn init(&mut self, cfg: &config::Config) -> Option<ErrorKind> {
        match cfg.get::<Config>("db.glue") {
           Ok(val) => self.config = val, 
           Err(e) => return Some(ErrorKind::InitializationFailed(format!("failed to obtain configuration : {}", e)))
        };

        if !self.config.enabled {
            return None;
        }

        let storage = match SledStorage::new(self.config.uri.as_str()) {
            Ok(val) => val,
            Err(e) => return Some(ErrorKind::InitializationFailed(format!("failed to create new storage : {}", e)))
        };
        self.client = Some(Arc::new(Glue::new(storage)));

        None
    }
    fn close(&mut self) -> Option<ErrorKind> {
        todo!()
    } 

    async fn create(&mut self, character: Character) -> Result<Character, ErrorKind> {
        todo!()
    }
    async fn get_by_profile_id(&mut self, profile_id: String) -> Result<Character, ErrorKind> {
        todo!()
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
    async fn remove_by_profile_id(&mut self, character: Character) -> Result<Character, ErrorKind> {
        todo!()
    }
}
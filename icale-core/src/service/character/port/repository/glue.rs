use super::*;
use async_trait::async_trait;
use gluesql::{parse, Glue, SledStorage};

use std::boxed::Box;

pub struct GlueRepository {
    client: Option<Box<Glue>>,
}

impl GlueRepository {
    pub fn new() -> Self {
        Self{
            client: Option::None,
        }
    }
}

#[async_trait]
impl Repository for GlueRepository {
    fn init(&mut self, cfg: &config::Config) -> std::option::Option<ErrorKind> {
        todo!()
    }
    fn close(&mut self) -> std::option::Option<ErrorKind> {
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

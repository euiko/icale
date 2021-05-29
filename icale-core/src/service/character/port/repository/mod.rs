
use async_trait::async_trait;
use crate::service::character::domain::*;
use chrono::DateTime;
use std::result::Result;
use std::option::Option;
use config::*;

pub mod glue;

pub enum ErrorKind {
    Driver(String),
    NotEnabled,
    NotFound,
    PermissionDenied,
}

pub struct FindParams {
    profile_id: String,
    profile_name: String,
    keyword: String,

    skip: i64,
    limit: Option<i64>,
}

pub struct FindResult {
    records: Vec<Character>,
    total_records: i64,
    total_pages: Option<i64>,
}


#[async_trait]
pub trait Repository {
    fn init(&mut self, config: &Config) -> Option<ErrorKind>;
    fn close(&mut self) -> Option<ErrorKind>;

    async fn create(&mut self, character: Character) -> Result<Character, ErrorKind>;
    async fn get_by_profile_id(&mut self, profile_id: String) -> Result<Character, ErrorKind>;
    async fn find(&mut self, params: FindParams) -> Result<FindResult, ErrorKind>;
    async fn update(&mut self, character: Character) -> Result<Character, ErrorKind>;
    async fn patch(&mut self, character: Character) -> Result<Character, ErrorKind>;
    async fn remove(&mut self, character: Character) -> Result<Character, ErrorKind>;
    async fn remove_by_profile_id(&mut self, character: Character) -> Result<Character, ErrorKind>;
}
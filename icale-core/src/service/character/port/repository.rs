use crate::service::character::domain::*;

use std::fmt::Display;
use std::result::Result;
use std::option::Option;

use async_trait::async_trait;
use config::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ErrorKind {
    Driver(String),
    InitializationFailed(String),
    NotEnabled,
    NotFound,
    PermissionDenied,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> { 
        match self {
            ErrorKind::Driver(str) => write!(f, "driver implementation error : {}", str),
            ErrorKind::InitializationFailed(str) => write!(f, "error occured when initializing repository : {}", str),
            ErrorKind::NotEnabled => write!(f, "repository is not enabled"),
            ErrorKind::NotFound => write!(f, "related data is not found"),
            ErrorKind::PermissionDenied => write!(f, "permission denied to access this resource"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FindParams {
    pub profile_id: Option<String>,
    pub profile_name: Option<String>,
    pub keyword: Option<String>,

    pub skip: i64,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct FindResult {
    pub records: Vec<Character>,
    pub total_records: i64,
    pub total_pages: Option<i64>,
}


#[async_trait(?Send)]
pub trait Repository {
    fn init(&mut self, config: &Config) -> Option<ErrorKind>;
    fn close(&mut self) -> Option<ErrorKind>;

    async fn create(&mut self, character: Character) -> Result<Character, ErrorKind>;
    async fn get_by_profile_id(&mut self, profile_id: &str) -> Result<Character, ErrorKind>;
    async fn find(&mut self, params: FindParams) -> Result<FindResult, ErrorKind>;
    async fn update(&mut self, character: Character) -> Result<Character, ErrorKind>;
    async fn patch(&mut self, character: Character) -> Result<Character, ErrorKind>;
    async fn remove(&mut self, character: Character) -> Result<Character, ErrorKind>;
    async fn remove_by_profile_id(&mut self, profile_id: &str) -> Result<Character, ErrorKind>;
}
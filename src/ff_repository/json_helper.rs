use std::fs::read_to_string;
use serde::Deserialize;
use crate::ff_repository::repository_error::RepositoryError;

pub(super) struct JsonHelper;

impl JsonHelper {
    pub(crate) fn read_db(db_path: &str) -> Result<String, RepositoryError> {
        match read_to_string(db_path) {
            Ok(string_data) => return Ok(string_data),
            Err(_) => return Err(RepositoryError::FileNotFound),
        };
    }

    pub(crate) fn parse_data<'de, T>(json: &'de str) -> Result<T, RepositoryError> where T: Deserialize<'de> {
        match serde_json::from_str(&json) as serde_json::Result<T> {
            Ok(parse_result) => { Ok(parse_result) },
            Err(_) => { Err(RepositoryError::JSONDecoding) }
        }
    }
}
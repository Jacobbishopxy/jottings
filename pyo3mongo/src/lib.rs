//! Pyo3Mongo

pub mod db;
pub mod model;
pub mod service;

use thiserror::Error;

#[allow(dead_code)]
pub type Pyo3MongoResult<T> = Result<T, Pyo3MongoError>;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Pyo3MongoError {
    #[error("common error {0}")]
    Common(&'static str),

    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),

    #[error(transparent)]
    De(#[from] bson::de::Error),
}

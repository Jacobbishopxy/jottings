//! Pyo3Mongo

pub mod db;
pub mod model;
pub mod service;

pub use model::*;
pub use service::GraphService;

use thiserror::Error;

pub type Pyo3MongoResult<T> = Result<T, Pyo3MongoError>;

#[derive(Error, Debug)]
pub enum Pyo3MongoError {
    #[error("common error {0}")]
    Common(&'static str),

    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),

    #[error(transparent)]
    De(#[from] bson::de::Error),

    #[error(transparent)]
    Oid(#[from] bson::oid::Error),
}

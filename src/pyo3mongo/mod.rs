//! pyo3 mongo demo
//!
//! This is a demo of using pyo3 with mongodb.

use thiserror::Error;

pub mod db;
pub mod model;
pub mod service;

pub type Pyo3MongoResult<T> = Result<T, Pyo3MongoError>;

#[derive(Error, Debug)]
pub enum Pyo3MongoError {
    #[error("common error {0}")]
    Common(&'static str),

    #[error(transparent)]
    Mongo(#[from] mongodb::error::Error),
}

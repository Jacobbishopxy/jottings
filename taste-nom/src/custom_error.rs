//! Custom Error
//!
//! Turn Nom error into custom error

use nom::error::{ErrorKind, ParseError};
use thiserror::Error;

#[derive(Debug)]
pub struct CustomError(pub String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<(&'a str, ErrorKind)> for CustomError {
    fn from(error: (&'a str, ErrorKind)) -> Self {
        CustomError(format!("error code was: {error:?}"))
    }
}

impl<'a> ParseError<&'a str> for CustomError {
    fn from_error_kind(_: &'a str, kind: ErrorKind) -> Self {
        CustomError(format!("error code was: {kind:?}"))
    }

    fn append(_: &'a str, kind: ErrorKind, other: CustomError) -> Self {
        CustomError(format!("{other:?}\nerror code was: {kind:?}"))
    }
}

#[derive(Error, Debug)]
pub enum TasteNomError {
    #[error(transparent)]
    CustomNom(nom::Err<CustomError>),

    #[error("sql error: {0}")]
    Sql(String),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl From<nom::Err<CustomError>> for TasteNomError {
    fn from(error: nom::Err<CustomError>) -> Self {
        TasteNomError::CustomNom(error)
    }
}

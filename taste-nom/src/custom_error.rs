//! Custom Error
//!
//! Turn Nom error into custom error

use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1};
use nom::combinator::{recognize, rest, value};
use nom::error::{ErrorKind, ParseError};
use nom::sequence::{delimited, pair, separated_pair, tuple};
use nom::Err::Error;
use nom::IResult;
use thiserror::Error;

#[derive(Debug)]
pub struct NomError(String);

impl std::fmt::Display for NomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<(&'a str, ErrorKind)> for NomError {
    fn from(error: (&'a str, ErrorKind)) -> Self {
        NomError(format!("error code was: {:?}", error))
    }
}

impl<'a> ParseError<&'a str> for NomError {
    fn from_error_kind(_: &'a str, kind: ErrorKind) -> Self {
        NomError(format!("error code was: {:?}", kind))
    }

    fn append(_: &'a str, kind: ErrorKind, other: NomError) -> Self {
        NomError(format!("{:?}\nerror code was: {:?}", other, kind))
    }
}

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("custom nom error: {0}")]
    CustomNom(NomError),

    #[error("sql error: {0}")]
    Sql(String),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Unknown")]
    Unknown,
}

impl From<nom::Err<NomError>> for CustomError {
    fn from(e: nom::Err<NomError>) -> Self {
        CustomError::CustomNom(NomError(e.to_string()))
    }
}

#[derive(Debug, Clone)]
pub enum SqlBuilder {
    Mysql,
    Postgres,
    Sqlite,
}

impl FromStr for SqlBuilder {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mysql" | "m" => Ok(SqlBuilder::Mysql),
            "postgres" | "p" => Ok(SqlBuilder::Postgres),
            "sqlite" | "s" => Ok(SqlBuilder::Sqlite),
            _ => Err(CustomError::Sql(format!("unknown database type: {}", s))),
        }
    }
}

pub struct SqlConnInfo {
    pub driver: SqlBuilder,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub database: String,
}

impl SqlConnInfo {
    pub fn new(
        driver: SqlBuilder,
        username: &str,
        password: &str,
        host: &str,
        port: u32,
        database: &str,
    ) -> SqlConnInfo {
        SqlConnInfo {
            driver,
            username: username.to_owned(),
            password: password.to_owned(),
            host: host.to_owned(),
            port,
            database: database.to_owned(),
        }
    }
}

type NomResult<'a> = IResult<&'a str, (&'a str, &'a str), NomError>;

// fn get_conn_info(value: &str) -> IResult<&str, (&str, &str), NomError> {
fn get_conn_info(value: &str) -> Result<(), CustomError> {
    // let mut f_user_info = recognize(separated_pair(alphanumeric1, tag(":"), alphanumeric1));
    // let mut f_addr_info = recognize(separated_pair(rest, tag(":"), digit1));
    // let mut f_db_info = recognize(separated_pair(f_addr_info, tag("/"), alpha1));
    // let mut par2 = recognize(separated_pair(f_user_info, tag("@"), rest));
    let mut par = separated_pair(alpha1, tag("://"), rest);

    // if let Ok((_, (driver, db_info))) = par(value) {
    //     if let Ok((user_info, db_info)) = par2(db_info) {
    //         if let Ok((username, password)) = f_user_info(user_info) {
    //             if let Ok((host_port, database)) = f_db_info(db_info) {
    //                 if let Ok((host, port)) = f_addr_info(host_port) {
    //                     let driver = SqlBuilder::from_str(driver)?;
    //                     let port = port.parse::<u32>()?;

    //                     let conn_info =
    //                         SqlConnInfo::new(driver, &username, &password, &host, port, &database);

    //                     return Ok(());
    //                 }
    //             }
    //         }
    //     }

    //     return Ok(());
    // }

    let foo: NomResult = par(value);

    match foo {
        Ok(s) => todo!(),
        Err(e) => Err(e.into()),
    }

    // return Ok(());

    // Ok(())

    // Ok(("", ("", "")))
    // par(value)
    // Ok(())
}

#[test]
fn test_custom_error() {
    const ConnPattern: &str = "{}://{}:{}@{}:{}/{}";

    const Conn: &str = "mysql://root:root@localhost:3306/test";
}

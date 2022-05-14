//！Database connection string handling

use std::str::FromStr;

use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{alpha1, alphanumeric1, digit1};
use nom::combinator::rest;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::custom_error::{CustomError, TasteNomError};

#[derive(Debug, Clone)]
pub enum SqlBuilder {
    Mysql,
    Postgres,
    Sqlite,
}

impl FromStr for SqlBuilder {
    type Err = TasteNomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mysql" => Ok(SqlBuilder::Mysql),
            "postgres" => Ok(SqlBuilder::Postgres),
            "sqlite" => Ok(SqlBuilder::Sqlite),
            _ => Err(TasteNomError::Sql(format!("unknown database type: {}", s))),
        }
    }
}

#[derive(Debug)]
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

#[allow(dead_code)]
fn take_driver_and_rest(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("://"), rest)(input)
}

#[allow(dead_code)]
fn take_username_and_rest(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alphanumeric1, tag(":"), rest)(input)
}

#[allow(dead_code)]
fn take_password_and_rest(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alphanumeric1, tag("@"), rest)(input)
}

#[allow(dead_code)]
fn take_host_and_rest(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(":"), rest)(input)
}

#[allow(dead_code)]
fn take_port_and_database(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(digit1, tag("/"), rest)(input)
}

type ResultInfo1<'a> = (&'a str, (&'a str, (&'a str, (&'a str, (&'a str, &'a str)))));

#[allow(dead_code)]
fn get_conn_info1(value: &str) -> IResult<&str, ResultInfo1, CustomError> {
    let f_port_and_database = separated_pair(digit1, tag("/"), alphanumeric1);
    let f_host_and_rest = separated_pair(alpha1, tag(":"), f_port_and_database);
    let f_password_and_rest = separated_pair(alphanumeric1, tag("@"), f_host_and_rest);
    let f_username_and_rest = separated_pair(alphanumeric1, tag(":"), f_password_and_rest);
    let mut f_driver_and_rest = separated_pair(alpha1, tag("://"), f_username_and_rest);

    f_driver_and_rest(value)
}

type ResultInfo2<'a> = (&'a str, ((&'a str, &'a str), ((&'a str, &'a str), &'a str)));

#[allow(dead_code)]
fn get_conn_info2(value: &str) -> IResult<&str, ResultInfo2, CustomError> {
    let f_host_and_port = separated_pair(alpha1, tag(":"), digit1);
    let f_address_and_database = separated_pair(f_host_and_port, tag("/"), alphanumeric1);
    let f_username_and_password = separated_pair(alphanumeric1, tag(":"), alphanumeric1);
    let f_user_and_rest = separated_pair(f_username_and_password, tag("@"), f_address_and_database);
    let mut f_driver_and_rest = separated_pair(alpha1, tag("://"), f_user_and_rest);

    f_driver_and_rest(value)
}

type ConnStrPattern<'a> = (
    &'a str,
    (&'a str, ((&'a str, &'a str), ((&'a str, &'a str), &'a str))),
);

impl<'a> TryFrom<ConnStrPattern<'a>> for SqlConnInfo {
    type Error = TasteNomError;

    fn try_from(source: ConnStrPattern<'a>) -> Result<Self, Self::Error> {
        let (_, (driver, ((username, password), ((host, port), database)))) = source;

        Ok(Self::new(
            SqlBuilder::from_str(driver)?,
            username,
            password,
            host,
            port.parse::<u32>()?,
            database,
        ))
    }
}

#[allow(dead_code)]
type GeneralResult<T> = Result<T, TasteNomError>;

#[allow(dead_code)]
fn get_conn_info3(value: &str) -> GeneralResult<SqlConnInfo> {
    let f_host_and_port = separated_pair(take_until1(":"), tag(":"), digit1);
    let f_address_and_database = separated_pair(f_host_and_port, tag("/"), alphanumeric1);
    let f_username_and_password = separated_pair(alphanumeric1, tag(":"), alphanumeric1);
    let f_user_and_rest = separated_pair(f_username_and_password, tag("@"), f_address_and_database);
    let mut f_driver_and_rest = separated_pair(alpha1, tag("://"), f_user_and_rest);

    let res = f_driver_and_rest(value)?;

    SqlConnInfo::try_from(res)
}

#[cfg(test)]
mod custom_error_tests {
    use super::*;

    // connection string pattern:
    // {}://{}:{}@{}:{}/{}

    const CONN1: &str = "mysql://root:root@localhost:3306/test";
    const CONN2: &str = "mysql://root:root@127.0.0.1:3306/test";

    #[test]
    fn driver_and_rest() {
        let foo = take_driver_and_rest(CONN1);
        assert!(foo.is_ok());
        println!("{:?}", foo);
    }

    #[test]
    fn username_and_rest() {
        let foo = take_username_and_rest("root:root@localhost:3306/test");
        assert!(foo.is_ok());
        println!("{:?}", foo);
    }

    #[test]
    fn password_and_rest() {
        let foo = take_password_and_rest("root@localhost:3306/test");
        assert!(foo.is_ok());
        println!("{:?}", foo);
    }

    #[test]
    fn host_and_rest() {
        let foo = take_host_and_rest("localhost:3306/test");
        assert!(foo.is_ok());
        println!("{:?}", foo);
    }

    #[test]
    fn port_and_db() {
        let foo = take_port_and_database("3306/test");
        assert!(foo.is_ok());
        println!("{:?}", foo);
    }

    #[test]
    fn conn_info() {
        let foo = get_conn_info1(CONN1);
        assert!(foo.is_ok());
        println!("{:?}", foo);

        let foo = get_conn_info2(CONN1);
        assert!(foo.is_ok());
        println!("{:?}", foo);

        let foo = get_conn_info3(CONN1);
        assert!(foo.is_ok());
        println!("{:?}", foo);

        let foo = get_conn_info3(CONN2);
        assert!(foo.is_ok());
        println!("{:?}", foo);
    }
}

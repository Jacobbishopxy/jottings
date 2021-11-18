//! taste_nom

use std::collections::HashMap;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha0, alpha1, space0};
use nom::combinator::recognize;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

// error
#[derive(Debug)]
pub(crate) enum ParsingError {
    InvalidDbType(String),
    InvalidDataType(String, String),
    ParsingError(String),
}

// database type
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DbType {
    Mysql,
    Postgres,
    Sqlite,
}

// str -> database type
impl FromStr for DbType {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MYSQL" => Ok(DbType::Mysql),
            "POSTGRES" => Ok(DbType::Postgres),
            "SQLITE" => Ok(DbType::Sqlite),
            _ => Err(ParsingError::InvalidDbType(s.to_string())),
        }
    }
}

// value type
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum ValueType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
}

lazy_static::lazy_static! {
    // str -> mysql data types
    pub(crate) static ref MYSQL_TMAP: HashMap<&'static str, ValueType> = {
        HashMap::from([
            ("TINYINT(1)", ValueType::Bool),
            ("BOOLEAN", ValueType::Bool),
            ("TINYINT UNSIGNED", ValueType::U8),
            ("SMALLINT UNSIGNED", ValueType::U16),
            ("INT UNSIGNED", ValueType::U32),
            ("BIGINT UNSIGNED", ValueType::U64),
            ("TINYINT", ValueType::I8),
            ("SMALLINT", ValueType::I16),
            ("INT", ValueType::I32),
            ("BIGINT", ValueType::I64),
            ("FLOAT", ValueType::F32),
            ("DOUBLE", ValueType::F64),
            ("VARCHAR", ValueType::String),
            ("CHAR", ValueType::String),
            ("TEXT", ValueType::String),
        ])
    };

    // str -> postgres data types
    pub(crate) static ref POSTGRES_TMAP: HashMap<&'static str, ValueType> = {
        HashMap::from([
            ("BOOL", ValueType::Bool),
            ("CHAR", ValueType::I8),
            ("TINYINT", ValueType::I8),
            ("SMALLINT", ValueType::I16),
            ("SMALLSERIAL", ValueType::I16),
            ("INT2", ValueType::I16),
            ("INT", ValueType::I32),
            ("SERIAL", ValueType::I32),
            ("INT4", ValueType::I32),
            ("BIGINT", ValueType::I64),
            ("BIGSERIAL", ValueType::I64),
            ("INT8", ValueType::I64),
            ("REAL", ValueType::F32),
            ("FLOAT4", ValueType::F32),
            ("DOUBLE PRECISION", ValueType::F64),
            ("FLOAT8", ValueType::F64),
            ("VARCHAR", ValueType::String),
            ("CHAR(N)", ValueType::String),
            ("TEXT", ValueType::String),
            ("NAME", ValueType::String),
        ])
    };

    // str -> sqlite data types
    pub(crate) static ref SQLITE_TMAP: HashMap<&'static str, ValueType> = {
        HashMap::from([
            ("BOOLEAN", ValueType::Bool),
            ("INTEGER", ValueType::I32),
            ("BIGINT", ValueType::I64),
            ("INT8", ValueType::I64),
            ("REAL", ValueType::F64),
            ("VARCHAR", ValueType::String),
            ("CHAR(N)", ValueType::String),
            ("TEXT", ValueType::String),
        ])
    };
}

#[test]
fn test_get_tmap() {
    assert_eq!(MYSQL_TMAP.get("BIGINT UNSIGNED").unwrap(), &ValueType::U64);
    assert_eq!(POSTGRES_TMAP.get("REAL").unwrap(), &ValueType::F32);
    assert_eq!(SQLITE_TMAP.get("CHAR(N)").unwrap(), &ValueType::String);
}

// ------------------------------------------------------------------------------

// parse database and data type
fn get_types(input: &str) -> IResult<&str, (&str, &str)> {
    let sql_type = |s| alpha1(s);
    let data_type = |s| alpha1(s);

    let ctn = separated_pair(sql_type, tag(":"), data_type);
    let mut par = delimited(tag("["), ctn, tag("]"));

    par(input)
}

#[test]
fn test_get_types() {
    assert_eq!(
        get_types("[MYSQL:BOOLEAN]").unwrap(),
        ("", ("MYSQL", "BOOLEAN"))
    );

    // this cannot be achieved because there is a space between "DOUBLE" and "PRECISION"
    // assert_eq!(
    //     get_types("[POSTGRES:DOUBLE PRECISION]").unwrap(),
    //     ("", ("POSTGRES", "DOUBLE PRECISION"))
    // );
}

fn get_types2(input: &str) -> IResult<&str, (&str, &str)> {
    let sql_type = |s| alpha1(s);
    let data_type = |s| recognize(separated_pair(alpha1, space0, alpha0))(s);

    let ctn = separated_pair(sql_type, tag(":"), data_type);
    let mut par = delimited(tag("["), ctn, tag("]"));

    par(input)
}

#[test]
fn test_get_types2() {
    assert_eq!(
        get_types2("[MYSQL:BOOLEAN]").unwrap(),
        ("", ("MYSQL", "BOOLEAN"))
    );

    assert_eq!(
        get_types2("[POSTGRES:DOUBLE PRECISION]").unwrap(),
        ("", ("POSTGRES", "DOUBLE PRECISION"))
    );

    // this cannot be achieved because there is a "(N)" cannot be recognized
    // assert_eq!(
    //     get_types2("[SQLITE:CHAR(N)]").unwrap(),
    //     ("", ("SQLITE", "CHAR(N)"))
    // );
}

fn get_types3(input: &str) -> IResult<&str, (&str, &str)> {
    let sql_type = |s| alpha1(s);
    let data_type = |s| take_until("]")(s);
    let ctn = separated_pair(sql_type, tag(":"), data_type);
    let mut par = delimited(tag("["), ctn, tag("]"));

    par(input)
}

#[test]
fn test_get_types3() {
    assert_eq!(
        get_types3("[MYSQL:BOOLEAN]").unwrap(),
        ("", ("MYSQL", "BOOLEAN"))
    );

    assert_eq!(
        get_types3("[POSTGRES:DOUBLE PRECISION]").unwrap(),
        ("", ("POSTGRES", "DOUBLE PRECISION"))
    );

    assert_eq!(
        get_types3("[SQLITE:CHAR(N)]").unwrap(),
        ("", ("SQLITE", "CHAR(N)"))
    );
}

// str -> (DbType, ValueType)
fn from_str_to_type(input: &str) -> Result<(DbType, ValueType), ParsingError> {
    match get_types3(input) {
        Ok((_, (db_type, data_type))) => match db_type.parse::<DbType>() {
            Ok(dt) => {
                let rvt = match dt {
                    DbType::Mysql => {
                        MYSQL_TMAP
                            .get(data_type)
                            .ok_or(ParsingError::InvalidDataType(
                                "MYSQL".to_string(),
                                data_type.to_string(),
                            ))
                    }
                    DbType::Postgres => {
                        POSTGRES_TMAP
                            .get(data_type)
                            .ok_or(ParsingError::InvalidDataType(
                                "POSTGRES".to_string(),
                                data_type.to_string(),
                            ))
                    }
                    DbType::Sqlite => {
                        SQLITE_TMAP
                            .get(data_type)
                            .ok_or(ParsingError::InvalidDataType(
                                "SQLITE".to_string(),
                                data_type.to_string(),
                            ))
                    }
                };

                match rvt {
                    Ok(vt) => Ok((dt, vt.clone())),
                    Err(_) => Err(ParsingError::InvalidDataType(
                        db_type.to_string(),
                        data_type.to_string(),
                    )),
                }
            }
            Err(_) => Err(ParsingError::InvalidDbType(db_type.to_string())),
        },
        _ => Err(ParsingError::ParsingError(input.to_string())),
    }
}

#[test]
fn test_cvt() {
    assert_eq!(
        from_str_to_type("[MYSQL:BOOLEAN]").unwrap(),
        (DbType::Mysql, ValueType::Bool)
    );

    assert_eq!(
        from_str_to_type("[POSTGRES:DOUBLE PRECISION]").unwrap(),
        (DbType::Postgres, ValueType::F64)
    );

    assert_eq!(
        from_str_to_type("[SQLITE:CHAR(N)]").unwrap(),
        (DbType::Sqlite, ValueType::String)
    );
}

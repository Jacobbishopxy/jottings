//! Connector

use std::future::Future;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use sqlx::mssql::{MssqlPool, MssqlPoolOptions};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Database, FromRow, Mssql, MySql, Pool, Postgres};

pub enum DB {
    MsSql,
    MySql,
    Postgres,
}

impl FromStr for DB {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mssql" => Ok(DB::MsSql),
            "mysql" => Ok(DB::MySql),
            "postgres" => Ok(DB::Postgres),
            _ => Err(anyhow!(
                "must be one of the following: mssql/mysql/postgres"
            )),
        }
    }
}

const CONN_E_ERR: &str = "Castigate has already been connected!";
const CONN_N_ERR: &str = "Castigate has not establish a connection yet!";

pub type PipeFn<I, O> = fn(I) -> Result<O>;

/// Connector
///
/// Database connector which supports Mssql/Mysql/Postgres.
pub struct Connector<T: SqlMeta> {
    conn_str: String,
    pool_options: Option<T>,
}

impl<T: SqlMeta> Connector<T> {
    pub fn new<S: Into<String>>(conn_str: S) -> Self {
        Self {
            conn_str: conn_str.into(),
            pool_options: None,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        match self.pool_options.as_ref() {
            None => {
                let p = T::new(&self.conn_str).await?;
                self.pool_options = Some(p);
                Ok(())
            }
            Some(_) => Err(anyhow!(CONN_E_ERR)),
        }
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        match self.pool_options.take() {
            None => Err(anyhow!(CONN_N_ERR)),
            Some(p) => p.close().await,
        }
    }

    pub async fn query<'a, D>(
        &'a self,
        sql: &'a str,
        pipe: PipeFn<<T::DB as Database>::Row, D>,
    ) -> Result<Vec<D>>
    where
        D: Send + Unpin + 'a,
    {
        match self.pool_options.as_ref() {
            Some(p) => p.query(sql, pipe).await,
            None => Err(anyhow!(CONN_N_ERR)),
        }
    }

    pub async fn query_one<'a, D>(
        &'a self,
        sql: &'a str,
        pipe: PipeFn<<T::DB as Database>::Row, D>,
    ) -> Result<D>
    where
        D: Send + Unpin + 'a,
    {
        match self.pool_options.as_ref() {
            Some(p) => p.query_one(sql, pipe).await,
            None => Err(anyhow!(CONN_N_ERR)),
        }
    }

    pub async fn query_as<'a, D>(&'a self, sql: &'a str) -> Result<Vec<D>>
    where
        D: Send + Unpin + for<'r> FromRow<'r, <T::DB as Database>::Row>,
    {
        match self.pool_options.as_ref() {
            Some(p) => p.query_as(sql).await,
            None => Err(anyhow!(CONN_N_ERR)),
        }
    }

    pub async fn query_one_as<'a, D>(&'a self, sql: &'a str) -> Result<D>
    where
        D: Send + Unpin + for<'r> FromRow<'r, <T::DB as Database>::Row>,
    {
        match self.pool_options.as_ref() {
            Some(p) => p.query_one_as(sql).await,
            None => Err(anyhow!(CONN_N_ERR)),
        }
    }
}

pub trait SqlMeta: Sized {
    // async function's return for self constructor
    type FutSelf<'a>: Future<Output = Result<Self>>
    where
        Self: 'a;

    // async function's return without return's type
    type FutNil<'a>: Future<Output = Result<()>>
    where
        Self: 'a;

    // trait from `sqlx`, only accepts `Mssql`/`MySql`/`Postgres`
    type DB: Database;

    // constructor
    fn new(conn_str: &str) -> Self::FutSelf<'_>;

    // close connection
    fn close(&self) -> Self::FutNil<'_>;

    // check if connection is closed
    fn is_closed(&self) -> bool;

    // query with a pipe function handling with `Database::Row`
    fn query<'a, D>(
        &'a self,
        sql: &'a str,
        pipe: PipeFn<<Self::DB as Database>::Row, D>,
    ) -> BoxFuture<'a, Result<Vec<D>>>
    where
        D: Send + Unpin + 'a;

    // query (limit one)
    fn query_one<'a, D>(
        &'a self,
        sql: &'a str,
        pipe: PipeFn<<Self::DB as Database>::Row, D>,
    ) -> BoxFuture<'a, Result<D>>
    where
        D: Send + Unpin + 'a;

    // query with an explicit type announcement, who implemented `FrowRow`
    fn query_as<'a, D>(&'a self, sql: &'a str) -> BoxFuture<'a, Result<Vec<D>>>
    where
        D: Send + Unpin + for<'r> FromRow<'r, <Self::DB as Database>::Row>;

    // query (limit one)
    fn query_one_as<'a, D>(&'a self, sql: &'a str) -> BoxFuture<'a, Result<D>>
    where
        D: Send + Unpin + for<'r> FromRow<'r, <Self::DB as Database>::Row>;

    // execute SQL statement without output
    fn execute<'a>(
        &'a self,
        sql: &'a str,
    ) -> BoxFuture<'a, Result<<Self::DB as Database>::QueryResult>>;
}

macro_rules! impl_sql_meta {
    ($db:ident, $db_pool_options:ident, $db_pool:ident) => {
        impl SqlMeta for Pool<$db> {
            type FutSelf<'a> = impl Future<Output = Result<Self>>;
            type FutNil<'a> = impl Future<Output = Result<()>>;
            type DB = $db;

            fn new(conn_str: &str) -> Self::FutSelf<'_> {
                async move {
                    let po = $db_pool_options::new().connect(conn_str).await?;
                    Ok(po)
                }
            }

            fn close(&self) -> Self::FutNil<'_> {
                async move {
                    $db_pool::close(self).await;
                    Ok(())
                }
            }

            fn is_closed(&self) -> bool {
                $db_pool::is_closed(self)
            }

            fn query<'a, T: Send + Unpin + 'a>(
                &'a self,
                sql: &'a str,
                pipe: PipeFn<<Self::DB as Database>::Row, T>,
            ) -> BoxFuture<'a, Result<Vec<T>>> {
                let q = async move {
                    sqlx::query(sql)
                        .try_map(|r| Ok(pipe(r).map_err(|e| anyhow!(e))))
                        .fetch_all(self)
                        .await
                        .map_err(|e| anyhow!(e))
                        .and_then(|r| r.into_iter().collect::<Result<Vec<T>>>())
                };
                Box::pin(q)
            }

            fn query_one<'a, T: Send + Unpin + 'a>(
                &'a self,
                sql: &'a str,
                pipe: PipeFn<<Self::DB as Database>::Row, T>,
            ) -> BoxFuture<'a, Result<T>> {
                let q = async move {
                    sqlx::query(sql)
                        .try_map(|r| Ok(pipe(r).map_err(|e| anyhow!(e))))
                        .fetch_one(self)
                        .await
                        .map_err(|e| anyhow!(e))
                        .and_then(|r| r)
                };
                Box::pin(q)
            }

            fn query_as<'a, T: Send + Unpin + for<'r> FromRow<'r, <Self::DB as Database>::Row>>(
                &'a self,
                sql: &'a str,
            ) -> BoxFuture<'a, Result<Vec<T>>> {
                let q = async move {
                    sqlx::query_as::<_, T>(sql)
                        .fetch_all(self)
                        .await
                        .map_err(|e| anyhow!(e))
                };
                Box::pin(q)
            }

            fn query_one_as<
                'a,
                T: Send + Unpin + for<'r> FromRow<'r, <Self::DB as Database>::Row>,
            >(
                &'a self,
                sql: &'a str,
            ) -> BoxFuture<'a, Result<T>> {
                let q = async move {
                    sqlx::query_as::<_, T>(sql)
                        .fetch_one(self)
                        .await
                        .map_err(|e| anyhow!(e))
                };
                Box::pin(q)
            }

            fn execute<'a>(
                &'a self,
                sql: &'a str,
            ) -> BoxFuture<'a, Result<<Self::DB as Database>::QueryResult>> {
                let q = async move { sqlx::query(sql).execute(self).await.map_err(|e| anyhow!(e)) };
                Box::pin(q)
            }
        }
    };
}

impl_sql_meta!(Mssql, MssqlPoolOptions, MssqlPool);
impl_sql_meta!(MySql, MySqlPoolOptions, MySqlPool);
impl_sql_meta!(Postgres, PgPoolOptions, PgPool);

#[cfg(test)]
mod test_connector {
    use sqlx::{postgres::PgRow, Row};

    use super::*;

    const URL: &str = "postgres://root:secret@localhost:5432/dev";

    #[tokio::test]
    async fn query_success() {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct User {
            email: String,
            nickname: String,
            hash: String,
            role: String,
        }

        impl User {
            fn new(email: String, nickname: String, hash: String, role: String) -> Self {
                User {
                    email,
                    nickname,
                    hash,
                    role,
                }
            }

            fn from_pg_row(row: PgRow) -> Result<Self> {
                let email: String = row.try_get(0)?;
                let nickname: String = row.try_get(1)?;
                let hash: String = row.try_get(2)?;
                let role: String = row.try_get(3)?;

                Ok(Self::new(email, nickname, hash, role))
            }
        }

        let mut ct = Connector::<PgPool>::new(URL);

        ct.connect().await.expect("Connection success");

        let sql = "SELECT * FROM users";

        let res = ct.query(sql, User::from_pg_row).await;

        println!("{:?}", res);

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn query_as_success() {
        #[allow(dead_code)]
        #[derive(sqlx::FromRow, Debug)]
        struct Users {
            email: String,
            nickname: String,
            hash: String,
            role: String,
        }

        let mut ct = Connector::<PgPool>::new(URL);
        ct.connect().await.expect("Connection success");

        let sql = "SELECT * FROM users";

        let res = ct.query_as::<Users>(sql).await;

        println!("{:?}", res);

        assert!(res.is_ok());
    }
}

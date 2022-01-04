//! MongoDB

use mongodb::{error::Error as MongoError, options::ClientOptions, Client};

pub type MongoResult<T> = Result<T, MongoError>;

pub(crate) struct MongoClient {
    client: mongodb::Client,
    db: String,
}

impl MongoClient {
    pub async fn new(uri: &str, db: &str) -> MongoResult<MongoClient> {
        let mut co = ClientOptions::parse(uri).await?;
        co.app_name = Some("pyo3mongo".to_owned());

        let client = Client::with_options(co)?;
        Ok(MongoClient {
            client,
            db: db.to_owned(),
        })
    }

    pub async fn show_dbs(&self) -> MongoResult<Vec<String>> {
        let db_names = self.client.list_database_names(None, None).await?;

        Ok(db_names)
    }

    /// specify which collection to be operated, and what schema
    /// is to be used (by generic parameter `T`)
    pub fn collection<T>(&self, name: &str) -> mongodb::Collection<T> {
        self.client.database(&self.db).collection(name)
    }
}

#[cfg(test)]
mod pyo3mongo_tests {
    use super::*;

    const URI: &str = "mongodb://root:secret@localhost:27017";
    const DB: &str = "graph";

    #[tokio::test]
    async fn test_mongo_client() {
        let client = MongoClient::new(URI, DB).await.unwrap();
        let db_names = client.show_dbs().await.unwrap();

        println!("{:?}", db_names);
    }
}

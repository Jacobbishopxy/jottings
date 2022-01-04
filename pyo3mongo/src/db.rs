//! MongoDB

use mongodb::{error::Error as MongoError, options::ClientOptions, Client};

#[allow(dead_code)]
const DB_NAME: &str = "graph";

#[allow(dead_code)]
pub type MongoResult<T> = Result<T, MongoError>;

#[allow(dead_code)]
pub struct MongoClient {
    client: mongodb::Client,
}

#[allow(dead_code)]
impl MongoClient {
    pub async fn new(uri: &str) -> MongoResult<MongoClient> {
        let mut co = ClientOptions::parse(uri).await?;
        co.app_name = Some("pyo3mongo".to_owned());

        let client = Client::with_options(co)?;
        Ok(MongoClient { client })
    }

    pub fn client(&self) -> mongodb::Client {
        self.client.clone()
    }

    pub async fn show_dbs(&self) -> MongoResult<Vec<String>> {
        let db_names = self.client.list_database_names(None, None).await?;

        Ok(db_names)
    }

    /// specify which collection to be operated, and what schema
    /// is to be used (by generic parameter `T`)
    pub fn collection<T>(&self, name: &str) -> mongodb::Collection<T> {
        self.client.database(DB_NAME).collection(name)
    }
}

#[cfg(test)]
mod pyo3mongo_tests {
    use super::*;

    const URI: &'static str = "mongodb://root:secret@localhost:27017";

    #[tokio::test]
    async fn test_mongo_client() {
        let client = MongoClient::new(URI).await.unwrap();
        let db_names = client.show_dbs().await.unwrap();

        println!("{:?}", db_names);
    }
}

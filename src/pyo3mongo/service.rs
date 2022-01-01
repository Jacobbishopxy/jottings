//! Service
//!

use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, Document};

use super::db::MongoClient;
use super::model::{Edge, Vertex};
use super::{Pyo3MongoError, Pyo3MongoResult};

pub struct GraphService {
    client: MongoClient,
    cat: String,
}

impl GraphService {
    pub async fn new(uri: &str, cat: &str) -> Pyo3MongoResult<Self> {
        Ok(GraphService {
            client: MongoClient::new(uri).await?,
            cat: cat.to_owned(),
        })
    }

    pub async fn create_vertex(&self, name: &str) -> Pyo3MongoResult<Vertex> {
        let vertex = Vertex {
            id: None,
            cat: self.cat.clone(),
            name: name.to_owned(),
        };

        let result = self
            .client
            .collection::<Vertex>(&self.cat)
            .insert_one(vertex, None)
            .await?;

        let id = result.inserted_id.as_object_id().unwrap();
        let res = self
            .client
            .collection::<Vertex>(&self.cat)
            .find_one(Some(bson::doc! {"_id": id}), None)
            .await?
            .ok_or(Pyo3MongoError::Common("vertex not found"));

        Ok(res?)
    }

    pub async fn get_vertex(&self, id: ObjectId) -> Pyo3MongoResult<Vertex> {
        let res = self
            .client
            .collection::<Vertex>(&self.cat)
            .find_one(Some(bson::doc! {"_id": id}), None)
            .await?
            .ok_or(Pyo3MongoError::Common("vertex not found"));

        Ok(res?)
    }

    pub async fn update_vertex(&self, id: ObjectId, vertex: Vertex) -> Pyo3MongoResult<Vertex> {
        let id = bson::doc! {"_id": id};
        let update = bson::doc! {
            "$set": Document::from(&vertex)
        };
        let res = self
            .client
            .collection::<Vertex>(&self.cat)
            .find_one_and_update(id, update, None)
            .await?
            .ok_or(Pyo3MongoError::Common("vertex not found"));

        Ok(res?)
    }
}

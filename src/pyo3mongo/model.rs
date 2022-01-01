//! model
//!
//! Creation of a graph:
//! 1. Create two `Vertex`s `v1` and `v2`
//! 2. Create an `Edge` that connects `v1` and `v2`

use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

/// edge between two vertices
#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub cat: String,
    pub source: ObjectId,
    pub target: ObjectId,
    pub weight: Option<f64>,
    pub label: Option<String>,
}

/// vertex
#[derive(Serialize, Deserialize, Debug)]
pub struct Vertex {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub cat: String,
    pub name: String,
}

impl From<&Edge> for Document {
    fn from(source: &Edge) -> Self {
        bson::to_document(source).unwrap()
    }
}

impl From<&Vertex> for Document {
    fn from(source: &Vertex) -> Self {
        bson::to_document(source).unwrap()
    }
}

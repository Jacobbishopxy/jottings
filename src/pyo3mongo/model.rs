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

/// DTO for `Edge`
#[derive(Serialize, Deserialize)]
pub struct EdgeDto<'a> {
    pub source: ObjectId,
    pub target: ObjectId,
    pub weight: Option<f64>,
    pub label: Option<&'a str>,
}

impl<'a> EdgeDto<'a> {
    pub fn to_edge(&self, cat: &str) -> Edge {
        Edge {
            id: None,
            cat: cat.to_owned(),
            source: self.source.to_owned(),
            target: self.target.to_owned(),
            weight: self.weight,
            label: self.label.map(str::to_string),
        }
    }
}

/// DTO for `Vertex`
#[derive(Serialize, Deserialize)]
pub struct VertexDto<'a> {
    pub name: &'a str,
}

impl<'a> VertexDto<'a> {
    pub fn to_vertex(&self, cat: &str) -> Vertex {
        Vertex {
            id: None,
            cat: cat.to_owned(),
            name: self.name.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FindEdgeByVertexDto {
    Source(ObjectId),
    Target(ObjectId),
    Bidirectional(ObjectId),
}

impl FindEdgeByVertexDto {
    pub fn id(&self) -> ObjectId {
        match self {
            FindEdgeByVertexDto::Source(id) => id.to_owned(),
            FindEdgeByVertexDto::Target(id) => id.to_owned(),
            FindEdgeByVertexDto::Bidirectional(id) => id.to_owned(),
        }
    }
}

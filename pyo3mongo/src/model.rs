//! model
//!
//! Creation of a graph:
//! 1. Create two `Vertex`s `v1` and `v2`
//! 2. Create an `Edge` that connects `v1` and `v2`

use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PureId {
    #[serde(rename = "_id")]
    pub id: ObjectId,
}

/// edge between two vertices
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Edge {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub source: ObjectId,
    pub target: ObjectId,
    pub weight: Option<f64>,
    pub label: Option<String>,
}

/// vertex
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Vertex {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EdgeDto<'a> {
    pub source: ObjectId,
    pub target: ObjectId,
    pub weight: Option<f64>,
    pub label: Option<&'a str>,
}

impl<'a> EdgeDto<'a> {
    pub fn new(
        source: ObjectId,
        target: ObjectId,
        weight: Option<f64>,
        label: Option<&'a str>,
    ) -> Self {
        EdgeDto {
            source,
            target,
            weight,
            label,
        }
    }
}

impl<'a> From<EdgeDto<'a>> for Edge {
    fn from(source: EdgeDto<'a>) -> Self {
        Edge {
            id: None,
            source: source.source,
            target: source.target,
            weight: source.weight,
            label: source.label.map(str::to_string),
        }
    }
}

/// DTO for `Vertex`
#[derive(Serialize, Deserialize, Debug)]
pub struct VertexDto<'a> {
    pub name: &'a str,
}

impl<'a> VertexDto<'a> {
    pub fn new(name: &'a str) -> Self {
        VertexDto { name }
    }
}

impl<'a> From<VertexDto<'a>> for Vertex {
    fn from(source: VertexDto<'a>) -> Self {
        Vertex {
            id: None,
            name: source.name.to_string(),
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

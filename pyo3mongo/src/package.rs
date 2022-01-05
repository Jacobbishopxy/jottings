//! Pyo3 Async

use std::str::FromStr;

use bson::oid::ObjectId;
use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

use crate::{Edge, EdgeDto, GraphService, Pyo3MongoError, Vertex, VertexDto};

impl From<Pyo3MongoError> for PyErr {
    fn from(e: Pyo3MongoError) -> Self {
        PyBaseException::new_err(e.to_string())
    }
}

#[pyclass]
pub struct PyGraph {
    service: GraphService,
    runtime: Runtime,
}

impl<'a> FromPyObject<'a> for VertexDto<'a> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let v: PyResult<VertexDto> = ob.extract();
        Ok(v?)
    }
}

pub struct EdgeInput {
    pub source: String,
    pub target: String,
    pub weight: Option<f64>,
    pub label: Option<String>,
}

impl<'a> TryFrom<&'a EdgeInput> for EdgeDto<'a> {
    type Error = Pyo3MongoError;

    fn try_from(value: &'a EdgeInput) -> Result<Self, Self::Error> {
        let v = EdgeDto {
            source: ObjectId::from_str(&value.source)?,
            target: ObjectId::from_str(&value.target)?,
            weight: value.weight,
            label: value.label.as_deref(),
        };
        Ok(v)
    }
}

impl<'a> FromPyObject<'a> for EdgeInput {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let v: PyResult<EdgeInput> = ob.extract();
        Ok(v?)
    }
}

#[pymethods]
impl PyGraph {
    #[new]
    fn new_graph(uri: &str, database: &str, category: &str) -> PyResult<PyGraph> {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let service =
            runtime.block_on(async move { GraphService::new(uri, database, category).await })?;

        Ok(PyGraph { service, runtime })
    }

    pub fn create_vertex(&self, v: VertexDto) -> PyResult<Vertex> {
        let res = self
            .runtime
            .block_on(async { self.service.create_vertex(v).await })?;

        Ok(res)
    }

    pub fn create_edge(&self, v: EdgeInput) -> PyResult<Edge> {
        let dto = EdgeDto::try_from(&v)?;
        let res = self
            .runtime
            .block_on(async { self.service.create_edge(dto).await })?;

        Ok(res)
    }
}

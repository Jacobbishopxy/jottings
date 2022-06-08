//! Pyo3 Async

use std::str::FromStr;

use bson::oid::ObjectId;
use pyo3::exceptions::{PyBaseException, PyValueError};
use pyo3::prelude::*;
use tokio::runtime::Runtime;

use crate::{Edge, EdgeDto, GraphService, Pyo3MongoError, Vertex, VertexDto};

// turn Pyo3MongoError into PyResult
impl From<Pyo3MongoError> for PyErr {
    fn from(e: Pyo3MongoError) -> Self {
        PyBaseException::new_err(e.to_string())
    }
}

// getter & setter for Vertex
#[pymethods]
impl Vertex {
    #[getter]
    pub fn get_id(&self) -> PyResult<String> {
        // since `Vertex` is always returned by `create_vertex`,
        // which means `id` is always given by MongoDB,
        // it is always safe to unwrap the `id`
        Ok(self.id.unwrap().to_string())
    }

    #[getter]
    pub fn get_name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    #[setter]
    pub fn set_name(&mut self, name: &str) -> PyResult<()> {
        self.name = name.to_owned();
        Ok(())
    }
}

// getter & setter for Edge
#[pymethods]
impl Edge {
    #[getter]
    pub fn get_id(&self) -> PyResult<String> {
        // since `Edge` is always returned by `create_edge`,
        // which means `id` is always given by MongoDB,
        // it is always safe to unwrap the `id`
        Ok(self.id.unwrap().to_hex())
    }

    #[getter]
    pub fn get_source(&self) -> PyResult<String> {
        Ok(self.source.to_hex())
    }

    #[setter]
    pub fn set_source(&mut self, value: &str) -> PyResult<()> {
        let source = ObjectId::from_str(value);
        match source {
            Ok(oid) => {
                self.source = oid;
                Ok(())
            }
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[getter]
    pub fn get_target(&self) -> PyResult<String> {
        Ok(self.target.to_hex())
    }

    #[setter]
    pub fn set_target(&mut self, value: &str) -> PyResult<()> {
        let target = ObjectId::from_str(value);
        match target {
            Ok(oid) => {
                self.target = oid;
                Ok(())
            }
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[getter]
    pub fn get_weight(&self) -> PyResult<Option<f64>> {
        Ok(self.weight)
    }

    #[setter]
    pub fn set_weight(&mut self, value: f64) -> PyResult<()> {
        self.weight = Some(value);
        Ok(())
    }

    #[getter]
    pub fn get_label(&self) -> PyResult<Option<String>> {
        Ok(self.label.clone())
    }

    #[setter]
    pub fn set_label(&mut self, value: &str) -> PyResult<()> {
        self.label = Some(value.to_string());
        Ok(())
    }
}

#[pyclass]
pub struct PyGraph {
    service: GraphService,
    runtime: Runtime,
}

#[pyclass]
#[derive(FromPyObject)]
pub struct EdgeInput {
    #[pyo3(get, set)]
    pub source: String,
    #[pyo3(get, set)]
    pub target: String,
    #[pyo3(get, set)]
    pub weight: Option<f64>,
    #[pyo3(get, set)]
    pub label: Option<String>,
}

#[pymethods]
impl EdgeInput {
    #[new]
    fn new(source: String, target: String, weight: Option<f64>, label: Option<String>) -> Self {
        EdgeInput {
            source,
            target,
            weight,
            label,
        }
    }
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

#[pyclass]
pub struct GraphOutput {
    #[pyo3(get)]
    pub vertexes: Vec<Vertex>,
    #[pyo3(get)]
    pub edges: Vec<Edge>,
}

#[pymethods]
impl PyGraph {
    #[new]
    fn new_graph(uri: String, database: String, category: String) -> PyResult<PyGraph> {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let service =
            runtime.block_on(async move { GraphService::new(&uri, &database, &category).await })?;

        Ok(PyGraph { service, runtime })
    }

    pub fn create_vertex(&self, v: String) -> PyResult<Py<Vertex>> {
        let dto = VertexDto::new(&v);
        let res = self
            .runtime
            .block_on(async { self.service.create_vertex(dto).await })?;

        // Global Interpreter Lock
        let gil = Python::acquire_gil();
        let py = gil.python();
        Py::new(py, res)
    }

    pub fn create_edge(&self, v: EdgeInput) -> PyResult<Py<Edge>> {
        let dto = EdgeDto::try_from(&v)?;
        let res = self
            .runtime
            .block_on(async { self.service.create_edge(dto).await })?;

        let gil = Python::acquire_gil();
        let py = gil.python();
        Py::new(py, res)
    }

    pub fn get_graph(
        &self,
        vertex_id: String,
        label: Option<&str>,
        depth: Option<i32>,
    ) -> PyResult<Py<GraphOutput>> {
        let (edges, vertexes) = self.runtime.block_on(async {
            let oid = ObjectId::from_str(&vertex_id)?;
            self.service
                .get_graph_from_vertex_by_label(oid, label, depth)
                .await
        })?;

        let res = GraphOutput { vertexes, edges };

        let gil = Python::acquire_gil();
        let py = gil.python();
        Py::new(py, res)
    }
}

#[pymodule]
fn p3m(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vertex>()?;
    m.add_class::<Edge>()?;
    m.add_class::<EdgeInput>()?;
    m.add_class::<GraphOutput>()?;
    m.add_class::<PyGraph>()?;
    Ok(())
}

//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/08 23:12:16 Friday
//! brief:

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////

#[pyfunction]
fn rust_log() {
    log::info!("rust_log");
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
struct PA {
    #[pyo3(get, set)]
    key: i64,
    #[pyo3(get, set)]
    name: Option<String>,
    #[pyo3(get, set)]
    props: Vec<String>,
}

#[pymethods]
impl PA {
    #[new]
    #[pyo3(signature = (key, name, props))]
    fn new(key: i64, name: Option<String>, props: Vec<String>) -> Self {
        PA { key, name, props }
    }

    #[pyo3(text_signature = "($self)")]
    fn to_json(&self) -> String {
        serde_json::json!(&self).to_string()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[pyfunction]
fn rust_sleep(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        log::info!("start sleep...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        log::info!("end sleep!");

        Ok(())
    })
}

async fn sleep_print(secs: u64, mut value: PA) -> PA {
    tokio::time::sleep(std::time::Duration::from_secs(secs)).await;

    value.props.push(secs.to_string());

    value
}

#[pyfunction]
fn rust_sleep_print<'p>(py: Python<'p>, secs: &'p PyAny, value: &'p PyAny) -> PyResult<&'p PyAny> {
    let secs = secs.extract()?;
    let value = value.extract()?;
    pyo3_asyncio::tokio::future_into_py(py, async move { Ok(sleep_print(secs, value).await) })
}

#[pymodule]
fn pyo3async(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_wrapped(wrap_pyfunction!(rust_log))?;
    m.add_class::<PA>()?;
    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(rust_sleep_print, m)?)?;

    Ok(())
}

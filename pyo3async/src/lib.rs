//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/08 23:12:16 Friday
//! brief:

use pyo3::prelude::*;

#[pyfunction]
fn rust_sleep(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        log::info!("start sleep...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        log::info!("end sleep!");

        Ok(())
    })
}

#[pyfunction]
fn rust_log() {
    log::info!("rust_log");
}

#[pymodule]
fn pyo3async(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    m.add_wrapped(wrap_pyfunction!(rust_log))?;

    Ok(())
}

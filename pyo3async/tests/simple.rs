//! file: simple.rs
//! author: Jacob Xie
//! date: 2023/09/08 23:04:52 Friday
//! brief:

use pyo3::prelude::*;

// simple test case
#[pyo3_asyncio::tokio::main]
async fn simple() -> PyResult<()> {
    let fut = Python::with_gil(|py| {
        let asyncio = py.import("asyncio")?;

        pyo3_asyncio::tokio::into_future(asyncio.call_method1("sleep", (1.into_py(py),))?)
    })?;

    fut.await?;

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////

use crate::dataset::nav::fetch_nav_history_bulk_eager;
use crate::schema::nav::DateRange;
use pyo3::{pyclass, pymethods, PyErr, PyResult};
use pyo3_polars::PyDataFrame;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

// Define a static accessor for the runtime
fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
}

/// Now your wrapper is extremely clean and DRY
fn run_async<T>(fut: impl std::future::Future<Output = T>) -> T {
    runtime().block_on(fut)
}

#[pyclass]
pub struct NavPipe {
    max_concurrency: usize,
}

#[pymethods]
impl NavPipe {
    #[new]
    fn new(max_concurrency: usize) -> Self {
        Self { max_concurrency }
    }

    #[pyo3(signature = (scheme_codes, start_date=None, end_date=None))]
    fn nav_history(
        &self,
        scheme_codes: Vec<u32>,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> PyResult<PyDataFrame> {
        let dr = match (start_date, end_date) {
            (Some(start), Some(end)) => Some(DateRange {
                start_date: start,
                end_date: end,
            }),
            _ => None,
        };

        let res = run_async(fetch_nav_history_bulk_eager(
            scheme_codes,
            dr,
            self.max_concurrency,
        ))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(PyDataFrame(res))
    }
}

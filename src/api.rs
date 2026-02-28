use crate::dataset::nav::{fetch_nav_history_bulk, fetch_nav_history_bulk_eager};
use crate::schema::nav::DateRange;
use polars::prelude::*;
use pyo3::prelude::*;
use anyhow::Error;

/// Simple sync wrapper
fn run_async<T>(fut: impl std::future::Future<Output = T>) -> T {
    tokio::runtime::Runtime::new().unwrap().block_on(fut)
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

    /// Fetch NAV as LazyFrame
    fn fetch_nav_lazy(
        &self,
        scheme_codes: Vec<u32>,
        date_range: Option<DateRange>,
    ) -> PyResult<LazyFrame> {
        Ok(run_async(fetch_nav_history_bulk(
            &scheme_codes,
            date_range.as_ref(),
            self.max_concurrency,
        ))?)
    }

    /// Fetch NAV as DataFrame (eager)
    fn fetch_nav_eager(
        &self,
        scheme_codes: Vec<u32>,
        date_range: Option<DateRange>,
    ) -> PyResult<DataFrame> {
        Ok(run_async(fetch_nav_history_bulk_eager(
            &scheme_codes,
            date_range.as_ref(),
            self.max_concurrency,
        ))?)
    }
}
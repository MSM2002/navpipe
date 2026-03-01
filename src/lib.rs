use pyo3::prelude::*;

pub mod api;
pub mod dataset;
pub mod schema;
pub mod transform;
pub mod transport;

#[pymodule]
fn navpipe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<api::NavPipe>()?;
    Ok(())
}
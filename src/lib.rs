pub mod grpc;

use pyo3::prelude::*;

#[pymodule]
// must match the crate's name
fn vioux(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(grpc::client::request, m)?)?;

    Ok(())
}

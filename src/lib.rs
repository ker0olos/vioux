pub(crate) mod grpc;
pub(crate) mod utils;

use pyo3::prelude::*;

pub use grpc::proto::vioux_server::ViouxServer;
pub use grpc::server::ViouxService;

#[pymodule]
// must match the crate's name
fn vioux(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(grpc::client::request_frame, m)?)?;
    m.add_function(wrap_pyfunction!(grpc::client::update_frame, m)?)?;

    Ok(())
}

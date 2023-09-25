mod export;
mod grpc;
mod store;

//
pub use export::{export_to_mp3, export_to_mp4};
pub use grpc::proto::vioux_server::{Vioux, ViouxServer};
pub use grpc::{
    proto::{Audio, ColorType, Image, RequestOptions},
    server::ViouxService,
};
//

use pyo3::prelude::*;

#[pymodule] // export to the python vioux library
fn vioux(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(grpc::client::request_frame, m)?)?;
    m.add_function(wrap_pyfunction!(grpc::client::update_frame, m)?)?;
    m.add_function(wrap_pyfunction!(grpc::client::append_frame, m)?)?;

    m.add_function(wrap_pyfunction!(grpc::client::request_audio, m)?)?;
    m.add_function(wrap_pyfunction!(grpc::client::update_audio, m)?)?;

    Ok(())
}

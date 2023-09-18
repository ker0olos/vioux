mod grpc;
mod render;
mod store;

use pyo3::prelude::*;

// export because it's required by main.rs or/and tests/*
pub use grpc::proto::vioux_server::{Vioux, ViouxServer};
pub use grpc::{
    proto::{Audio, ColorType, Image, RequestOptions},
    server::ViouxService,
};

#[pymodule]
// must match the crate's name
// used by the python scripting library
fn vioux(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    //
    m.add_function(wrap_pyfunction!(grpc::client::request_frame, m)?)?;
    m.add_function(wrap_pyfunction!(grpc::client::update_frame, m)?)?;

    // TODO
    // m.add_function(wrap_pyfunction!(grpc::client::request_audio, m)?)?;
    // m.add_function(wrap_pyfunction!(grpc::client::update_audio, m)?)?;

    Ok(())
}

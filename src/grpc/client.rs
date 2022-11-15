use numpy::{ndarray::Array, IntoPyArray};
use pyo3::prelude::*;

use super::proto::{image_client::ImageClient, FrameRequest};

#[pyfunction]
pub fn request(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let mut client = ImageClient::connect("http://0.0.0.0:50051").await.unwrap();

        let request = tonic::Request::new(FrameRequest {});

        let response = client.next_frame(request).await.unwrap();

        let ndarray = Array::from_vec(response.into_inner().frame);

        Ok(Python::with_gil(|py| {
            PyObject::from(ndarray.into_pyarray(py))
        }))
    })
}

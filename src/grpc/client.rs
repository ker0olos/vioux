use pyo3::prelude::*;

use numpy::{IntoPyArray, PyArray3};

use crate::utils::processing::into_ndarray3;

// All code here is for the python scripting library
// and is not used directly in/by vioux

use super::proto::{
    vioux_client::ViouxClient, Image, RequestFrameParameters, UpdateFrameParameters,
};

async fn connect() -> ViouxClient<tonic::transport::Channel> {
    ViouxClient::connect("http://0.0.0.0:50051").await.unwrap()
}

#[pyfunction]
pub fn request_frame(py: Python) -> PyResult<&PyAny> {
    let request = tonic::Request::new(RequestFrameParameters::default());

    pyo3_asyncio::tokio::future_into_py(py, async move {
        let mut client = connect().await;

        let response = client.request_frame(request).await.unwrap();

        let ndarray = into_ndarray3(response.into_inner().image.unwrap());

        Ok(Python::with_gil(|py| {
            PyObject::from(ndarray.into_pyarray(py))
        }))
    })
}

#[pyfunction]
pub fn update_frame(py: Python, image: PyObject) -> PyResult<&PyAny> {
    let ndarray = image.extract::<&PyArray3<u8>>(py)?;

    let shape = ndarray.shape();
    let raw = ndarray.to_vec().unwrap();

    let request = tonic::Request::new(UpdateFrameParameters {
        image: Some(Image {
            raw,
            height: shape[0] as u32,
            width: shape[1] as u32,
            channels: shape[2] as u32,
        }),
    });

    pyo3_asyncio::tokio::future_into_py(py, async move {
        let mut client = connect().await;
        client.update_frame(request).await.unwrap();
        Ok(())
    })
}

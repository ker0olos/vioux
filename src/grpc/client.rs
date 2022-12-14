use pyo3::{prelude::*, types::PyByteArray};

use numpy::{IntoPyArray, PyArray3};

use super::{
    proto::{vioux_client::ViouxClient, Image, RequestOptions},
    utils::{get_color_type, image_to_ndarray},
};

// All code here is for the python scripting library
// and is NOT to be used directly in or by vioux

async fn connect() -> ViouxClient<tonic::transport::Channel> {
    ViouxClient::connect("http://0.0.0.0:50051")
        .await
        .expect("Failed to connect to server")
}

#[pyfunction]
pub fn request_frame(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        let mut client = connect().await;

        let response = client
            .request_frame(RequestOptions::default())
            .await
            .expect("Request failed")
            .into_inner();

        let image = response.image.expect("Received an empty response");

        // use the raw image to get a ndarray
        let ndarray = image_to_ndarray(image)?;

        // convert the ndarray into an python numpy array and return it
        Ok(Python::with_gil(|py| {
            let array = ndarray.into_pyarray(py).to_object(py);
            array
        }))
    })
}

#[pyfunction]
pub fn update_frame(py: Python, image: PyObject) -> PyResult<&PyAny> {
    let ndarray = image.extract::<&PyArray3<u8>>(py)?;

    // try to find the color type of from the numpy array
    let color_type = get_color_type(ndarray)?;

    let shape = ndarray.shape();
    let raw = ndarray.to_vec()?;

    let request = RequestOptions {
        image: Some(Image {
            raw,
            width: shape[1] as u32,
            height: shape[0] as u32,
            color_type: color_type.into(),
        }),
    };

    // send the raw image to the server
    pyo3_asyncio::tokio::future_into_py(py, async {
        let mut client = connect().await;
        client.update_frame(request).await.expect("Request failed");
        Ok(())
    })
}

#[pyfunction]
pub fn request_audio(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        let mut client = connect().await;

        let response = client
            .request_audio(RequestOptions::default())
            .await
            .expect("Request failed")
            .into_inner();

        let audio = response.audio.expect("Received an empty response");

        // convert the ndarray into an python numpy array and return it
        Ok(Python::with_gil(|py| {
            let byte_array = PyByteArray::new(py, &audio.samples).to_object(py);
            (
                byte_array,
                audio.sample_rate,
                audio.sample_width,
                audio.channels,
            )
        }))
    })
}

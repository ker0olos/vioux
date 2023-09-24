use pyo3::{prelude::*, types::PyByteArray};

use super::{
    proto::{vioux_client::ViouxClient, Audio, RequestOptions},
    utils::{image_to_numpy, numpy_to_image},
};

// All code here is for the python scripting library
// and is NOT to be used directly in or by vioux

async fn connect() -> ViouxClient<tonic::transport::Channel> {
    ViouxClient::connect("http://0.0.0.0:50051")
        .await
        .expect("Failed to connect to server")
}

#[pyfunction]
pub fn request_frame(py: Python, n: u64) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let mut client = connect().await;

        let request = RequestOptions {
            n: Some(n),
            image: None,
            audio: None,
        };

        let response = client
            .request_frame(request)
            .await
            .expect("Request failed")
            .into_inner();

        let image = response.image.expect("Received an empty response");

        Ok(Python::with_gil(|py| image_to_numpy(image, py).unwrap()))
    })
}

#[pyfunction]
pub fn update_frame(py: Python, n: u64, image: PyObject, x: u32, y: u32) -> PyResult<&PyAny> {
    let image = numpy_to_image(image, py, x, y)?;

    let request = RequestOptions {
        n: Some(n),
        image: Some(image),
        audio: None,
    };

    pyo3_asyncio::tokio::future_into_py(py, async {
        let mut client = connect().await;
        client.update_frame(request).await.expect("Request failed");
        Ok(())
    })
}

#[pyfunction]
pub fn request_audio(py: Python, n: u64) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let mut client = connect().await;

        let response = client
            .request_audio(RequestOptions {
                n: Some(n),
                image: None,
                audio: None,
            })
            .await
            .expect("Request failed")
            .into_inner();

        let audio = response.audio.expect("Received an empty response");

        Ok(Python::with_gil(|py| {
            let byte_array = PyByteArray::new(py, &audio.data).to_object(py);
            (
                byte_array,
                audio.sample_rate,
                audio.sample_width,
                audio.channels,
                audio.codec,
            )
        }))
    })
}

#[pyfunction]
pub fn update_audio(
    py: Python,
    n: u64,
    data: Vec<u8>,
    sample_rate: u32,
    sample_width: u32,
    channels: u32,
    codec: String,
) -> PyResult<&PyAny> {
    let audio = Audio {
        data,
        sample_rate,
        sample_width,
        channels,
        codec,
    };

    let request = RequestOptions {
        n: Some(n),
        audio: Some(audio),
        image: None,
    };

    pyo3_asyncio::tokio::future_into_py(py, async {
        let mut client = connect().await;
        client.update_audio(request).await.expect("Request failed");
        Ok(())
    })
}

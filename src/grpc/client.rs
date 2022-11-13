use pyo3::prelude::*;

use super::placeholder::{placeholder_client::PlaceholderClient, PlaceholderRequest};

#[pyfunction]
pub fn request(py: Python, nonce: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let mut client = PlaceholderClient::connect("http://0.0.0.0:50051")
            .await
            .unwrap();

        let request = tonic::Request::new(PlaceholderRequest { nonce });

        let response = client.send_message(request).await.unwrap();

        let t = response.into_inner().nonce;

        Ok(t)
    })
}

use numpy::{
    ndarray::{Array3, Dim, ShapeBuilder},
    IntoPyArray,
};
use pyo3::prelude::*;

use super::proto::{vioux_client::ViouxClient, FrameRequestParameters, Image, SampleLayout};

fn into_ndarray3(
    sample_layout: SampleLayout,
    raw: Vec<u8>,
) -> numpy::ndarray::Array<u8, Dim<[usize; 3]>> {
    let SampleLayout {
        channels,
        channel_stride,
        height,
        height_stride,
        width,
        width_stride,
    } = sample_layout;

    let shape = (height as usize, width as usize, channels as usize);

    let strides = (
        height_stride as usize,
        width_stride as usize,
        channel_stride as usize,
    );

    println!("shape: {:?}", shape);
    println!("strides: {:?}", strides);

    Array3::from_shape_vec(shape.strides(strides), raw).unwrap()
}

#[pyfunction]
pub fn request(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let mut client = ViouxClient::connect("http://0.0.0.0:50051").await.unwrap();

        let request = tonic::Request::new(FrameRequestParameters {});

        let response = client.request_frame(request).await.unwrap();

        let Image { sample_layout, raw } = response.into_inner();

        let ndarray = into_ndarray3(sample_layout.unwrap(), raw);

        Ok(Python::with_gil(|py| {
            PyObject::from(ndarray.into_pyarray(py))
        }))
    })
}

use numpy::ndarray::{Array3, ShapeBuilder};
use pyo3::{
    exceptions::{PyNotImplementedError, PyTypeError},
    types::PyDict,
    PyResult,
};

use super::proto::{self, Image};

impl proto::ColorType {
    fn channel_count(&self) -> usize {
        match self {
            proto::ColorType::L8 | proto::ColorType::L16 => 1,
            proto::ColorType::La8 | proto::ColorType::La16 => 2,
            proto::ColorType::Rgb8 | proto::ColorType::Rgb16 | proto::ColorType::Rgb32F => 3,
            proto::ColorType::Rgba8 | proto::ColorType::Rgba16 | proto::ColorType::Rgba32F => 4,
        }
    }
}

pub fn into_array3(image: Image) -> PyResult<numpy::ndarray::Array3<u8>> {
    let channels = image.color_type().channel_count();

    let shape = (image.height as usize, image.width as usize, channels);

    if let Some(height_stride) = channels.checked_mul(image.width as usize) {
        let width_stride = channels;
        let strides = (height_stride, width_stride, 1);

        Ok(Array3::from_shape_vec(shape.strides(strides), image.raw).unwrap())
    } else {
        Err(PyTypeError::new_err(
            "Row major packed image can not be described because it does not fit into memory",
        ))
    }
}

pub fn get_color_type(ndarray: &numpy::PyArray3<u8>) -> PyResult<proto::ColorType> {
    let shape = ndarray.shape();

    let channels = shape[2];

    let typestr = ndarray
        .getattr("__array_interface__")?
        .extract::<&PyDict>()?
        .get_item("typestr")
        .unwrap()
        .extract::<&str>()?;

    match channels {
        // L
        1 => match typestr {
            // "|b1" => ("1", "1;8"),
            "|u1" => Ok(proto::ColorType::L8), // ("L", "L")
            // "|i1" => ("I", "I;8"),
            // "<u2" => ("I", "I;16"),
            // ">u2" => ("I", "I;16B"),
            // "<i2" => ("I", "I;16S"),
            // ">i2" => ("I", "I;16BS"),
            // "<u4" => ("I", "I;32"),
            // ">u4" => ("I", "I;32B"),
            // "<i4" => ("I", "I;32S"),
            // ">i4" => ("I", "I;32BS"),
            // "<f4" => ("F", "F;32F"),
            // ">f4" => ("F", "F;32BF"),
            // "<f8" => ("F", "F;64F"),
            // ">f8" => ("F", "F;64BF"),
            _ => Err(PyNotImplementedError::new_err("")),
        },
        // LA
        2 => match typestr {
            "|u1" => Ok(proto::ColorType::La8), // ("LA", "LA"),
            _ => Err(PyNotImplementedError::new_err("")),
        },
        // RGB
        3 => match typestr {
            "|u1" => Ok(proto::ColorType::Rgb8), //("RGB", "RGB"),
            _ => Err(PyNotImplementedError::new_err("")),
        },
        // RGBA
        4 => match typestr {
            "|u1" => Ok(proto::ColorType::Rgba8), //("RGBA", "RGBA"),
            _ => Err(PyNotImplementedError::new_err("")),
        },
        _ => Err(PyNotImplementedError::new_err("")),
    }
}

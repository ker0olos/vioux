use numpy::ndarray::{Array3, ShapeBuilder};

use pyo3::{
    exceptions::{PyNotImplementedError, PyTypeError},
    types::PyDict,
    PyResult,
};

use super::proto::{ColorType, Image};

// functions used by client.rs (executed inside python)

pub fn image_to_ndarray(image: Image) -> PyResult<numpy::ndarray::Array3<u8>> {
    let channels: usize = match image.color_type() {
        ColorType::L8 | ColorType::L16 => 1,
        ColorType::La8 | ColorType::La16 => 2,
        ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgb32F => 3,
        ColorType::Rgba8 | ColorType::Rgba16 | ColorType::Rgba32F => 4,
    };

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

pub fn get_color_type(ndarray: &numpy::PyArray3<u8>) -> PyResult<ColorType> {
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
            "|u1" => Ok(ColorType::L8), // ("L", "L")
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
            "|u1" => Ok(ColorType::La8), // ("LA", "LA"),
            _ => Err(PyNotImplementedError::new_err("")),
        },
        // RGB
        3 => match typestr {
            "|u1" => Ok(ColorType::Rgb8), //("RGB", "RGB"),
            _ => Err(PyNotImplementedError::new_err("")),
        },
        // RGBA
        4 => match typestr {
            "|u1" => Ok(ColorType::Rgba8), //("RGBA", "RGBA"),
            _ => Err(PyNotImplementedError::new_err("")),
        },
        _ => Err(PyNotImplementedError::new_err("")),
    }
}

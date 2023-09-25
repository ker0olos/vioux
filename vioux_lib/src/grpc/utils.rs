use numpy::{
    ndarray::{Array3, ShapeBuilder},
    IntoPyArray,
};

use pyo3::{
    exceptions::{PyNotImplementedError, PyTypeError},
    prelude::*,
    types::PyDict,
};
use symphonia::core::codecs::{
    CodecType, CODEC_TYPE_PCM_ALAW, CODEC_TYPE_PCM_F32BE, CODEC_TYPE_PCM_F32LE,
    CODEC_TYPE_PCM_F64BE, CODEC_TYPE_PCM_F64LE, CODEC_TYPE_PCM_MULAW, CODEC_TYPE_PCM_S16BE,
    CODEC_TYPE_PCM_S16LE, CODEC_TYPE_PCM_S24BE, CODEC_TYPE_PCM_S24LE, CODEC_TYPE_PCM_S32BE,
    CODEC_TYPE_PCM_S32LE, CODEC_TYPE_PCM_S8, CODEC_TYPE_PCM_U16BE, CODEC_TYPE_PCM_U16LE,
    CODEC_TYPE_PCM_U24BE, CODEC_TYPE_PCM_U24LE, CODEC_TYPE_PCM_U32BE, CODEC_TYPE_PCM_U32LE,
    CODEC_TYPE_PCM_U8,
};

use crate::grpc::proto::{ColorType, Image};

// functions used by client.rs (executed inside python)

pub fn image_to_numpy(image: Image, py: Python) -> PyResult<PyObject> {
    let channels: usize = match image.color_type() {
        ColorType::L8 | ColorType::L16 => 1,
        ColorType::La8 | ColorType::La16 => 2,
        ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Rgb32F => 3,
        ColorType::Rgba8 | ColorType::Rgba16 | ColorType::Rgba32F => 4,
    };

    let width_stride = channels;

    let shape = (image.height as usize, image.width as usize, channels);

    if let Some(height_stride) = channels.checked_mul(image.width as usize) {
        let shape = shape.strides((height_stride, width_stride, 1));

        let pyarray = match image.color_type() {
            ColorType::L8 | ColorType::La8 | ColorType::Rgb8 | ColorType::Rgba8 => {
                Array3::<u8>::from_shape_vec(shape, image.data)
                    .unwrap()
                    .into_pyarray(py)
                    .to_object(py)
            }
            ColorType::L16 | ColorType::La16 | ColorType::Rgb16 | ColorType::Rgba16 => {
                Array3::<u16>::from_shape_vec(shape, bytemuck::cast_vec::<u8, u16>(image.data))
                    .unwrap()
                    .into_pyarray(py)
                    .to_object(py)
            }
            ColorType::Rgb32F | ColorType::Rgba32F => {
                Array3::<f32>::from_shape_vec(shape, bytemuck::cast_vec::<u8, f32>(image.data))
                    .unwrap()
                    .into_pyarray(py)
                    .to_object(py)
            }
        };

        Ok(pyarray)
    } else {
        Err(PyTypeError::new_err(
            "Row major packed image can not be described because it does not fit into memory",
        ))
    }
}

pub fn numpy_to_image(
    ndarray: PyObject,
    py: Python,
    uuid: String,
    x: u32,
    y: u32,
) -> PyResult<Image> {
    let binding = ndarray.getattr(py, "__array_interface__")?;
    let interface = binding.extract::<&PyDict>(py)?;

    let shape = interface
        .get_item("shape")
        .unwrap()
        .extract::<(usize, usize, usize)>()?;

    let typestr = interface.get_item("typestr").unwrap().extract::<&str>()?;

    let channels = shape.2;

    let color_type = match channels {
        // L
        1 => match typestr {
            "|u1" => ColorType::L8,
            "<u2" => ColorType::L16,
            _ => return Err(PyNotImplementedError::new_err("")),
        },
        // La
        2 => match typestr {
            "|u1" => ColorType::La8,
            "<u2" => ColorType::La16,
            _ => return Err(PyNotImplementedError::new_err("")),
        },
        // Rgb
        3 => match typestr {
            "|u1" => ColorType::Rgb8,
            "<u2" => ColorType::Rgb16,
            "<f4" => ColorType::Rgb32F,
            _ => return Err(PyNotImplementedError::new_err("")),
        },
        // Rgba
        4 => match typestr {
            "|u1" => ColorType::Rgba8,
            "<u2" => ColorType::Rgba16,
            "<f4" => ColorType::Rgba32F,
            _ => return Err(PyNotImplementedError::new_err("")),
        },
        _ => return Err(PyNotImplementedError::new_err("")),
    };

    let data = match color_type {
        ColorType::L8 | ColorType::La8 | ColorType::Rgb8 | ColorType::Rgba8 => {
            let ndarray = ndarray.extract::<&numpy::PyArray3<u8>>(py)?;
            ndarray.to_vec()?
        }
        ColorType::L16 | ColorType::La16 | ColorType::Rgb16 | ColorType::Rgba16 => {
            let ndarray = ndarray.extract::<&numpy::PyArray3<u16>>(py)?;
            bytemuck::cast_vec(ndarray.to_vec()?)
        }
        ColorType::Rgb32F | ColorType::Rgba32F => {
            let ndarray = ndarray.extract::<&numpy::PyArray3<f32>>(py)?;
            bytemuck::cast_vec(ndarray.to_vec()?)
        }
    };

    let image = Image {
        data,
        width: shape.1 as u32,
        height: shape.0 as u32,
        color_type: color_type.into(),
        uuid,
        x,
        y,
    };

    Ok(image)
}

pub fn codec_to_string(codec: CodecType) -> &'static str {
    match codec {
        CODEC_TYPE_PCM_S32LE => "pcm_s32le",
        CODEC_TYPE_PCM_S32BE => "pcm_s32be",
        CODEC_TYPE_PCM_S24LE => "pcm_s24le",
        CODEC_TYPE_PCM_S24BE => "pcm_s24be",
        CODEC_TYPE_PCM_S16LE => "pcm_s16le",
        CODEC_TYPE_PCM_S16BE => "pcm_s16be",
        CODEC_TYPE_PCM_S8 => "pcm_s8",
        CODEC_TYPE_PCM_U32LE => "pcm_u32le",
        CODEC_TYPE_PCM_U32BE => "pcm_u32be",
        CODEC_TYPE_PCM_U24LE => "pcm_u24le",
        CODEC_TYPE_PCM_U24BE => "pcm_u24be",
        CODEC_TYPE_PCM_U16LE => "pcm_u16le",
        CODEC_TYPE_PCM_U16BE => "pcm_u16be",
        CODEC_TYPE_PCM_U8 => "pcm_u8",
        CODEC_TYPE_PCM_F32LE => "pcm_f32le",
        CODEC_TYPE_PCM_F32BE => "pcm_f32be",
        CODEC_TYPE_PCM_F64LE => "pcm_f64le",
        CODEC_TYPE_PCM_F64BE => "pcm_f64be",
        CODEC_TYPE_PCM_ALAW => "pcm_alaw",
        CODEC_TYPE_PCM_MULAW => "pcm_mulaw",
        _ => unimplemented!(),
    }
}

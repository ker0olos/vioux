use anyhow::Context;

use numpy::ndarray::{Array3, ShapeBuilder};

use pyo3::{
    exceptions::{PyNotImplementedError, PyTypeError},
    types::PyDict,
    PyResult,
};

use symphonia::core::{
    audio::{AudioBuffer, RawSampleBuffer},
    codecs::{Decoder, DecoderOptions},
    formats::{FormatOptions, FormatReader},
    io::{MediaSource, MediaSourceStream},
    meta::MetadataOptions,
    probe::Hint,
    sample::{i24, u24},
};

use super::proto::{self, Image};

pub trait ExtendWithBytes {
    fn extend_with_bytes(&self, samples: &mut Vec<u8>) -> ();
}

macro_rules! impl_extend_with_bytes {
    ($fmt:ty) => {
        impl ExtendWithBytes for AudioBuffer<$fmt> {
            fn extend_with_bytes(&self, samples: &mut Vec<u8>) {
                let mut byte_buf =
                    RawSampleBuffer::<$fmt>::new(self.capacity() as u64, *self.spec());

                byte_buf.copy_interleaved(self);

                // The interleaved samples can be accessed as a slice of bytes as follows.
                let bytes = byte_buf.as_bytes();

                samples.extend_from_slice(bytes);
            }
        }
    };
}

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

impl_extend_with_bytes!(u8);
impl_extend_with_bytes!(u16);
impl_extend_with_bytes!(u24);
impl_extend_with_bytes!(u32);
impl_extend_with_bytes!(i8);
impl_extend_with_bytes!(i16);
impl_extend_with_bytes!(i24);
impl_extend_with_bytes!(i32);
impl_extend_with_bytes!(f32);
impl_extend_with_bytes!(f64);

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

pub fn get_media_source(
    src: Box<dyn MediaSource>,
) -> anyhow::Result<(
    Box<dyn Decoder>,
    Box<dyn FormatReader>,
    u32,
    u32,
    u32,
    u32,
    u64,
)> {
    //
    let mss = MediaSourceStream::new(src, Default::default());

    // Create a probe hint using the file's extension
    let hint: Hint = Default::default();

    // Use the default options for metadata and format readers
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;

    let format = probed.format;

    let dec_opts: DecoderOptions = Default::default();

    let track = format.default_track().context("no default audio track")?;

    let track_id = track.id;

    let decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .context("unsupported codec")?;

    let sample_rate = track.codec_params.sample_rate.context("no sample rate")?;

    let sample_width = track
        .codec_params
        .bits_per_coded_sample
        .map(|bps| bps / 8)
        .context("no sample width")?;

    let channels = track
        .codec_params
        .channels
        .map(|c| c.count())
        .context("no channel count")?;

    let frame_length = track
        .codec_params
        .n_frames
        .context("cannot calculate duration")?;

    Ok((
        decoder,
        format,
        track_id,
        sample_rate,
        sample_width,
        channels as u32,
        frame_length,
    ))
}

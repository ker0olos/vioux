use anyhow::Context;

use uuid::Uuid;

use symphonia::core::{
    audio::{AudioBuffer, AudioBufferRef, RawSampleBuffer},
    codecs::DecoderOptions,
    errors::Error,
    formats::FormatOptions,
    io::{MediaSource, MediaSourceStream},
    meta::MetadataOptions,
    probe::Hint,
    sample::{i24, u24},
};

pub mod client;
pub mod server;

mod utils;

#[allow(clippy::all)]
pub mod proto {
    tonic::include_proto!("_");
}

mod image_types {
    use image::{Luma, LumaA, Rgb, Rgba};

    pub type RgbImage = image::ImageBuffer<Rgb<u8>, Vec<u8>>;
    pub type RgbaImage = image::ImageBuffer<Rgba<u8>, Vec<u8>>;
    pub type GrayImage = image::ImageBuffer<Luma<u8>, Vec<u8>>;
    pub type GrayAlphaImage = image::ImageBuffer<LumaA<u8>, Vec<u8>>;

    pub type Rgb16Image = image::ImageBuffer<Rgb<u16>, Vec<u16>>;
    pub type Rgba16Image = image::ImageBuffer<Rgba<u16>, Vec<u16>>;
    pub type Gray16Image = image::ImageBuffer<Luma<u16>, Vec<u16>>;
    pub type GrayAlpha16Image = image::ImageBuffer<LumaA<u16>, Vec<u16>>;

    pub type Rgb32FImage = image::ImageBuffer<Rgb<f32>, Vec<f32>>;
    pub type Rgba32FImage = image::ImageBuffer<Rgba<f32>, Vec<f32>>;
}

// functions used by server.rs

impl proto::ColorType {
    pub fn from(color_type: image::ColorType) -> proto::ColorType {
        match color_type {
            image::ColorType::L8 => proto::ColorType::L8,
            image::ColorType::La8 => proto::ColorType::La8,
            image::ColorType::Rgb8 => proto::ColorType::Rgb8,
            image::ColorType::Rgba8 => proto::ColorType::Rgba8,
            image::ColorType::L16 => proto::ColorType::L16,
            image::ColorType::La16 => proto::ColorType::La16,
            image::ColorType::Rgb16 => proto::ColorType::Rgb16,
            image::ColorType::Rgba16 => proto::ColorType::Rgba16,
            image::ColorType::Rgb32F => proto::ColorType::Rgb32F,
            image::ColorType::Rgba32F => proto::ColorType::Rgba32F,
            _ => unimplemented!(),
        }
    }
}

impl proto::Image {
    pub fn to_dynamic_image(self) -> anyhow::Result<image::DynamicImage> {
        let result = match self.color_type() {
            proto::ColorType::L8 => image::DynamicImage::from(
                image_types::GrayImage::from_raw(self.width, self.height, self.data).context("")?,
            ),
            proto::ColorType::La8 => image::DynamicImage::from(
                image_types::GrayAlphaImage::from_raw(self.width, self.height, self.data)
                    .context("")?,
            ),
            proto::ColorType::Rgb8 => image::DynamicImage::from(
                image_types::RgbImage::from_raw(self.width, self.height, self.data).context("")?,
            ),
            proto::ColorType::Rgba8 => image::DynamicImage::from(
                image_types::RgbaImage::from_raw(self.width, self.height, self.data).context("")?,
            ),
            proto::ColorType::L16 => image::DynamicImage::from(
                image_types::Gray16Image::from_raw(
                    self.width,
                    self.height,
                    bytemuck::cast_vec(self.data),
                )
                .context("")?,
            ),
            proto::ColorType::La16 => image::DynamicImage::from(
                image_types::GrayAlpha16Image::from_raw(
                    self.width,
                    self.height,
                    bytemuck::cast_vec(self.data),
                )
                .context("")?,
            ),
            proto::ColorType::Rgb16 => image::DynamicImage::from(
                image_types::Rgb16Image::from_raw(
                    self.width,
                    self.height,
                    bytemuck::cast_vec(self.data),
                )
                .context("")?,
            ),
            proto::ColorType::Rgba16 => image::DynamicImage::from(
                image_types::Rgba16Image::from_raw(
                    self.width,
                    self.height,
                    bytemuck::cast_vec(self.data),
                )
                .context("")?,
            ),
            proto::ColorType::Rgb32F => image::DynamicImage::from(
                image_types::Rgb32FImage::from_raw(
                    self.width,
                    self.height,
                    bytemuck::cast_vec(self.data),
                )
                .context("")?,
            ),
            proto::ColorType::Rgba32F => image::DynamicImage::from(
                image_types::Rgba32FImage::from_raw(
                    self.width,
                    self.height,
                    bytemuck::cast_vec(self.data),
                )
                .context("")?,
            ),
        };

        Ok(result)
    }
}

impl proto::Audio {
    pub fn from_media_source(src: Box<dyn MediaSource>) -> anyhow::Result<proto::Audio> {
        let mss = MediaSourceStream::new(src, Default::default());

        // Create a probe hint using the file's extension
        let hint: Hint = Default::default();

        // Use the default options for metadata and format readers
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;

        let mut format = probed.format;

        let dec_opts: DecoderOptions = Default::default();

        let track = format.default_track().context("no default audio track")?;

        let track_id = track.id;

        let mut decoder = symphonia::default::get_codecs()
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
            .map(|c| c.count() as u32)
            .context("no channel count")?;

        let codec = utils::codec_to_string(track.codec_params.codec).to_owned();

        let mut data = Vec::new();

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(Error::IoError(err))
                    if err.kind() == std::io::ErrorKind::UnexpectedEof
                        && err.to_string() == "end of stream" =>
                {
                    // Do not treat "end of stream" as a fatal error. It's the currently only way a
                    // format reader can indicate the media is complete
                    break;
                }
                Err(err) => return Err(anyhow::Error::from(err)),
            };

            // If the packet does not belong to the selected track, skip over it
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
                Ok(decoded) => match &decoded {
                    AudioBufferRef::U8(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::U16(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::U24(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::U32(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::S8(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::S16(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::S24(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::S32(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::F32(buf) => buf.extend_with_bytes(&mut data),
                    AudioBufferRef::F64(buf) => buf.extend_with_bytes(&mut data),
                },
                Err(Error::DecodeError(_)) => {
                    // Decode errors are not fatal. Print the error message and try to decode the next
                    // packet as usual
                    continue;
                }
                Err(err) => return Err(anyhow::Error::from(err)),
            }
        }

        // send a raw decoded audio to the client
        Ok(proto::Audio {
            data,
            sample_rate,
            sample_width,
            channels,
            codec,
            uuid: Uuid::new_v4().to_string(),
        })
    }
}

pub trait ExtendWithBytes {
    fn extend_with_bytes(&self, samples: &mut Vec<u8>);
}

macro_rules! impl_extend_with_bytes {
    ($fmt:ty) => {
        impl ExtendWithBytes for AudioBuffer<$fmt> {
            fn extend_with_bytes(&self, samples: &mut Vec<u8>) {
                let mut byte_buf =
                    RawSampleBuffer::<$fmt>::new(self.capacity() as u64, *self.spec());

                byte_buf.copy_interleaved(self);

                let bytes = byte_buf.as_bytes();

                samples.extend_from_slice(bytes);
            }
        }
    };
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

use std::path::PathBuf;

use image::DynamicImage;

use symphonia::core::{audio::AudioBufferRef, errors::Error};

use tonic::{Request, Response, Status};

use super::{
    proto::{
        vioux_server::Vioux, Audio, ColorType, Image, RequestOptions, RequestedAudio,
        RequestedFrame, UpdatedFrame,
    },
    utils::{get_media_source, ExtendWithBytes},
};

#[derive(Default)]
pub struct ViouxService {}

#[tonic::async_trait]
impl Vioux for ViouxService {
    async fn request_frame(
        &self,
        _request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedFrame>> {
        // TODO REMOVE placeholder
        let image = image::io::Reader::open("tests/assets/img.jpeg")
            .unwrap()
            .decode()
            .unwrap();

        let color_type: ColorType = image.color().into();

        // send a raw decoded image to the client
        let response = RequestedFrame {
            image: Some(Image {
                width: image.width(),
                height: image.height(),
                raw: image.into_bytes(),
                color_type: color_type.into(),
            }),
        };

        Ok(Response::new(response))
    }

    async fn update_frame(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedFrame>> {
        let image = request.into_inner().image.unwrap();

        // try to get a DynamicImage obj from the raw image
        let _image = match image.color_type() {
            ColorType::L8 => DynamicImage::from(
                image::GrayImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            ColorType::La8 => DynamicImage::from(
                image::GrayAlphaImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            ColorType::Rgb8 => DynamicImage::from(
                image::RgbImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            ColorType::Rgba8 => DynamicImage::from(
                image::RgbaImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            color_type @ _ => {
                return Err(Status::unimplemented(format!(
                    "{} is not supported yet!",
                    color_type.as_str_name()
                )))
            }
        };

        // image.save("img_export.jpeg").unwrap();

        Ok(Response::new(UpdatedFrame::default()))
    }

    async fn request_audio(
        &self,
        _request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedAudio>> {
        // TODO REMOVE placeholder
        let path = PathBuf::from("tests/assets/sound.wav");

        let src = std::fs::File::open(&path).unwrap();

        let (mut decoder, mut format, track_id, sample_rate, sample_width, channels, ..) =
            match get_media_source(Box::new(src)) {
                Ok(mms) => mms,
                Err(err) => return Err(Status::new(tonic::Code::Internal, err.to_string())),
            };

        let mut samples = Vec::new();

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
                Err(err) => return Err(Status::new(tonic::Code::InvalidArgument, err.to_string())),
            };

            // If the packet does not belong to the selected track, skip over it
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
                Ok(decoded) => match &decoded {
                    AudioBufferRef::U8(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::U16(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::U24(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::U32(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::S8(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::S16(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::S24(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::S32(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::F32(buf) => buf.extend_with_bytes(&mut samples),
                    AudioBufferRef::F64(buf) => buf.extend_with_bytes(&mut samples),
                },
                Err(Error::DecodeError(err)) => {
                    // Decode errors are not fatal. Print the error message and try to decode the next
                    // packet as usual
                    println!("decode error: {}", err)
                }
                Err(err) => return Err(Status::new(tonic::Code::Internal, err.to_string())),
            }
        }

        // send a raw decoded audio to the client
        let response = RequestedAudio {
            audio: Some(Audio {
                samples,
                sample_rate,
                sample_width,
                channels,
            }),
        };

        // clean up (drop) the decoder
        decoder.finalize();

        Ok(Response::new(response))
    }
}

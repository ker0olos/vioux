use std::path::PathBuf;

use image::DynamicImage;

use symphonia::core::{
    audio::{AudioBufferRef, RawSampleBuffer},
    codecs::DecoderOptions,
    errors::Error,
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};

use tonic::{Request, Response, Status};

use super::proto::{
    self, vioux_server::Vioux, Audio, Image, RequestOptions, RequestedAudio, RequestedFrame,
    UpdatedFrame,
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

        let color_type: proto::ColorType = image.color().into();

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
            proto::ColorType::L8 => DynamicImage::from(
                image::GrayImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            proto::ColorType::La8 => DynamicImage::from(
                image::GrayAlphaImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            proto::ColorType::Rgb8 => DynamicImage::from(
                image::RgbImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            proto::ColorType::Rgba8 => DynamicImage::from(
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

        let src = std::fs::File::open(&path).expect("failed to open media");

        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        // Create a probe hint using the file's extension.
        let hint: Hint = Default::default();

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        // Probe the media source.
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("unsupported format");

        let mut format = probed.format;

        let dec_opts: DecoderOptions = Default::default();

        // let track = format
        //     .tracks()
        //     .iter()
        //     .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        //     .expect("no supported audio tracks");

        let track = format.default_track().expect("no default audio track");

        let track_id = track.id;

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("unsupported codec");

        let sample_rate = track.codec_params.sample_rate.expect("no sample rate");

        // let sample_width = track.codec_params
        //     .bits_per_coded_sample
        //     .map(|bps| bps / 8)
        //     .expect("no sample width");

        let channels = track
            .codec_params
            .channels
            .map(|c| c.count())
            .expect("no channel count") as u32;

        let frame_length = track
            .codec_params
            .n_frames
            .expect("cannot calculate duration");

        let mut samples = Vec::new();

        let result = loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(Error::IoError(err))
                    if err.kind() == std::io::ErrorKind::UnexpectedEof
                        && err.to_string() == "end of stream" =>
                {
                    // Do not treat "end of stream" as a fatal error. It's the currently only way a
                    // format reader can indicate the media is complete.
                    break Ok(());
                }
                Err(err) => break Err(err),
            };

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
                Ok(decoded) => match &decoded {
                    AudioBufferRef::S16(buf) => {
                        let mut byte_buf =
                            RawSampleBuffer::<i16>::new(buf.capacity() as u64, *buf.spec());

                        // Copy the contents of the decoded audio buffer into the sample buffer whilst performing
                        // any required conversions.
                        byte_buf.copy_interleaved(buf.as_ref());

                        // The interleaved samples can be accessed as a slice of bytes as follows.
                        let bytes = byte_buf.as_bytes();

                        samples.extend_from_slice(bytes);
                    }
                    // TODO
                    _ => break Err(Error::Unsupported("unimplemented!")),
                },
                Err(Error::DecodeError(err)) => {
                    // Decode errors are not fatal. Print the error message and try to decode the next
                    // packet as usual.
                    println!("decode error: {}", err) // REMOVE
                }
                Err(err) => break Err(err),
            }
        };

        // TODO REMOVE
        if result.is_err() {
            panic!();
        }

        // REMOVE
        println!("samples: {:?}", samples.len());
        // println!("format: {:?}", sample_format);
        println!("sample_rate: {:?}", sample_rate);
        // println!("sample_width: {:?}", sample_width);
        println!("channels: {:?}", channels);
        println!("frame_length: {:?}", frame_length);

        // send a raw decoded image to the client
        let response = RequestedAudio {
            audio: Some(Audio {
                samples,
                sample_format: proto::SampleFormat::S16.into(), // TODO
                sample_rate,
                channels,
            }),
        };

        // clean up (drop) the decoder
        decoder.finalize();

        Ok(Response::new(response))
    }
}

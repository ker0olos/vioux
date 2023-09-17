use tonic::{Request, Response, Status};

use super::proto::{
    vioux_server::Vioux, Audio, ColorType, Image, RequestOptions, RequestedAudio, RequestedFrame,
    UpdatedAudio, UpdatedFrame,
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

        let color_type = ColorType::from(image.color());

        // send a raw decoded image to the client
        Ok(Response::new(RequestedFrame {
            image: Some(Image {
                width: image.width(),
                height: image.height(),
                color_type: color_type.into(),
                data: image.into_bytes(),
            }),
        }))
    }

    async fn update_frame(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedFrame>> {
        let image = request
            .into_inner()
            .image
            .and_then(|i| i.to_dynamic_image().ok());

        // TODO

        match image {
            Some(_image) => Ok(Response::new(UpdatedFrame::default())),
            None => Err(Status::new(tonic::Code::NotFound, "No image was received")),
        }
    }

    async fn request_audio(
        &self,
        _request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedAudio>> {
        // TODO REMOVE placeholder
        let src = std::fs::File::open("tests/assets/sound.wav").unwrap();

        let result = Audio::from_media_source(Box::new(src));

        match result {
            // send a raw decoded audio to the client
            Ok(audio) => Ok(Response::new(RequestedAudio { audio: Some(audio) })),
            Err(err) => Err(Status::new(tonic::Code::Aborted, err.to_string())),
        }
    }

    async fn update_audio(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedAudio>> {
        let audio = request.into_inner().audio;

        // TODO

        match audio {
            Some(_) => Ok(Response::new(UpdatedAudio::default())),
            None => Err(Status::new(tonic::Code::NotFound, "No audio was received")),
        }
    }
}

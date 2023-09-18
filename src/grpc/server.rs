use tonic::{Request, Response, Status};

use crate::{render::export_to_mp4, store::FRAMES};

use super::proto::{
    vioux_server::Vioux, RequestOptions, RequestedAudio, RequestedFrame, UpdatedAudio, UpdatedFrame,
};

#[derive(Default)]
pub struct ViouxService {}

#[tonic::async_trait]
impl Vioux for ViouxService {
    async fn request_frame(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedFrame>> {
        let request = request.into_inner();

        if let Some(n) = request.n {
            if let Some(image) = FRAMES.lock().unwrap().get(&n) {
                // send a raw decoded image to the client
                Ok(Response::new(RequestedFrame {
                    n,
                    image: Some(image.clone()),
                }))
            } else {
                Err(Status::new(
                    tonic::Code::NotFound,
                    "nth frame doesn't exist",
                ))
            }
        } else {
            Err(Status::new(
                tonic::Code::NotFound,
                "No nth frame was requested",
            ))
        }
    }

    async fn update_frame(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedFrame>> {
        let request = request.into_inner();

        if let Some(image) = request.image {
            if let Some(n) = request.n {
                FRAMES.lock().unwrap().insert(n, image);
                export_to_mp4();
                Ok(Response::new(UpdatedFrame::default()))
            } else {
                Err(Status::new(
                    tonic::Code::NotFound,
                    "No nth frame was received",
                ))
            }
        } else {
            Err(Status::new(tonic::Code::NotFound, "No image was received"))
        }
    }

    async fn request_audio(
        &self,
        _request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedAudio>> {
        // TODO

        // let src = std::fs::File::open("tests/assets/sound.wav").unwrap();

        // let result = Audio::from_media_source(Box::new(src));

        // match result {
        //     // send a raw decoded audio to the client
        //     Ok(audio) => Ok(Response::new(RequestedAudio { audio: Some(audio) })),
        //     Err(err) => Err(Status::new(tonic::Code::Aborted, err.to_string())),
        // }

        todo!()
    }

    async fn update_audio(
        &self,
        _request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedAudio>> {
        // TODO

        // let audio = request.into_inner().audio;

        // match audio {
        //     Some(_) => Ok(Response::new(UpdatedAudio::default())),
        //     None => Err(Status::new(tonic::Code::NotFound, "No audio was received")),
        // }

        todo!()
    }
}

use tonic::{Request, Response, Status};

use crate::store::{FRAMES, SEGMENTS};

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
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedAudio>> {
        let request = request.into_inner();

        if let Some(n) = request.n {
            if let Some(audio) = SEGMENTS.lock().unwrap().get(&n) {
                Ok(Response::new(RequestedAudio {
                    n,
                    audio: Some(audio.clone()),
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

    async fn update_audio(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedAudio>> {
        let request = request.into_inner();

        if let Some(audio) = request.audio {
            if let Some(n) = request.n {
                SEGMENTS.lock().unwrap().insert(n, audio);

                Ok(Response::new(UpdatedAudio::default()))
            } else {
                Err(Status::new(
                    tonic::Code::NotFound,
                    "No nth frame was received",
                ))
            }
        } else {
            Err(Status::new(tonic::Code::NotFound, "No audio was received"))
        }
    }
}

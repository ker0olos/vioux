use tonic::{Request, Response, Status};

use crate::store;

use crate::grpc::proto::{
    vioux_server::Vioux, AppendedFrame, RequestOptions, RequestedAudio, RequestedFrame,
    UpdatedAudio, UpdatedFrame,
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

        if let Some(layer) = request.layer {
            if let Some(n) = request.n {
                if let Some(image) = store::get_frames_by_layer(layer).get(n as usize) {
                    // send a raw decoded image to the client
                    Ok(Response::new(RequestedFrame {
                        n,
                        layer,
                        image: Some((*image).clone()),
                    }))
                } else {
                    Err(Status::new(
                        tonic::Code::NotFound,
                        "Requested frame doesn't exist",
                    ))
                }
            } else {
                Err(Status::new(tonic::Code::NotFound, "No frame was request"))
            }
        } else {
            Err(Status::new(tonic::Code::NotFound, "No layer was requested"))
        }
    }

    async fn update_frame(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedFrame>> {
        let request = request.into_inner();

        if let Some(image) = request.image {
            match store::update_frame(image) {
                Ok(_) => Ok(Response::new(UpdatedFrame::default())),
                Err(err) => Err(Status::new(tonic::Code::NotFound, err.to_string())),
            }
        } else {
            Err(Status::new(tonic::Code::NotFound, "No image was received"))
        }
    }

    async fn append_frame(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<AppendedFrame>> {
        let request = request.into_inner();

        if let Some(layer) = request.layer {
            if let Some(image) = request.image {
                let id = store::insert_frame_to_layer(layer, image);
                Ok(Response::new(AppendedFrame { id }))
            } else {
                Err(Status::new(tonic::Code::NotFound, "No image was received"))
            }
        } else {
            Err(Status::new(tonic::Code::NotFound, "No layer was specified"))
        }
    }

    async fn request_audio(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedAudio>> {
        let request = request.into_inner();

        if let Some(n) = request.n {
            if let Some(audio) = store::SEGMENTS.lock().unwrap().get(&n.into()) {
                Ok(Response::new(RequestedAudio {
                    n: n.into(),
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
                store::SEGMENTS.lock().unwrap().insert(n.into(), audio);

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

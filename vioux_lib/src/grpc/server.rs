use tonic::{Request, Response, Status};

use crate::store;

use crate::grpc::proto::{
    vioux_server::Vioux, AppendedFrame, RequestOptions, RequestedAudio, RequestedFrame,
    UpdatedAudio, UpdatedFrame,
};

use super::proto::AppendedAudio;

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

        if let Some(layer) = request.layer {
            if let Some(n) = request.n {
                if let Some(audio) = store::get_audio_by_layer(layer).get(n as usize) {
                    // send a raw decoded image to the client
                    Ok(Response::new(RequestedAudio {
                        n,
                        layer,
                        audio: Some((*audio).clone()),
                    }))
                } else {
                    Err(Status::new(
                        tonic::Code::NotFound,
                        "Requested audio doesn't exist",
                    ))
                }
            } else {
                Err(Status::new(tonic::Code::NotFound, "No frame was request"))
            }
        } else {
            Err(Status::new(tonic::Code::NotFound, "No layer was requested"))
        }
    }

    async fn update_audio(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedAudio>> {
        let request = request.into_inner();

        if let Some(audio) = request.audio {
            match store::update_audio(audio) {
                Ok(_) => Ok(Response::new(UpdatedAudio::default())),
                Err(err) => Err(Status::new(tonic::Code::NotFound, err.to_string())),
            }
        } else {
            Err(Status::new(tonic::Code::NotFound, "No audio was received"))
        }
    }

    async fn append_audio(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<AppendedAudio>> {
        let request = request.into_inner();

        if let Some(layer) = request.layer {
            if let Some(audio) = request.audio {
                let id = store::insert_audio_to_layer(layer, audio);
                Ok(Response::new(AppendedAudio { id }))
            } else {
                Err(Status::new(tonic::Code::NotFound, "No audio was received"))
            }
        } else {
            Err(Status::new(tonic::Code::NotFound, "No layer was specified"))
        }
    }
}

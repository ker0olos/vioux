use tonic::{transport::Server, Request, Response, Status};

use super::proto::{
    image_server::{Image, ImageServer},
    FrameRequest, FrameResponse,
};

#[derive(Default)]
struct ImageService {}

#[tonic::async_trait]
impl Image for ImageService {
    async fn next_frame(
        &self,
        _: Request<FrameRequest>,
    ) -> Result<Response<FrameResponse>, Status> {
        // TODO
        let placeholder = std::fs::read("img.jpeg").unwrap();

        let reply = FrameResponse { frame: placeholder };

        Ok(Response::new(reply))
    }
}

pub fn spawn() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;

    let image_service_impl = ImageService::default();

    tokio::spawn(async move {
        Server::builder()
            .add_service(ImageServer::new(image_service_impl))
            .serve(addr)
            .await
    });

    Ok(())
}

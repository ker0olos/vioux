use tonic::{transport::Server, Request, Response, Status};

use super::placeholder::{
    placeholder_server::{Placeholder, PlaceholderServer},
    PlaceholderRequest, PlaceholderResponse, FILE_DESCRIPTOR_SET,
};

#[derive(Debug, Default)]
struct MyPlaceholder {}

#[tonic::async_trait]
impl Placeholder for MyPlaceholder {
    async fn send_message(
        &self,
        request: Request<PlaceholderRequest>,
    ) -> Result<Response<PlaceholderResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = PlaceholderResponse {
            nonce: format!("Returning \"{}\"!", request.into_inner().nonce),
        };

        Ok(Response::new(reply))
    }
}

pub fn spawn() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;
    let placeholder = MyPlaceholder::default();

    let descriptor = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;

    tokio::spawn(async move {
        Server::builder()
            .add_service(descriptor)
            .add_service(PlaceholderServer::new(placeholder))
            .serve(addr)
            .await
    });

    Ok(())
}

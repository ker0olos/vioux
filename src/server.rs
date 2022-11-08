use tonic::{transport::Server, Request, Response, Status};

use placeholder::{
    placeholder_server::{Placeholder, PlaceholderServer},
    PlaceholderRequest, PlaceholderResponse,
};

#[derive(Debug, Default)]
struct MyPlaceholder {}

#[allow(clippy::all)]
mod placeholder {
    tonic::include_proto!("placeholder");
}

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

pub fn spawn() {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let placeholder = MyPlaceholder::default();

    let handle = tokio::runtime::Handle::current();

    handle.spawn(async move {
        Server::builder()
            .add_service(PlaceholderServer::new(placeholder))
            .serve(addr)
            .await
            .unwrap();
    });
}

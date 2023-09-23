use vioux::{App, ViouxServer, ViouxService};

use iced::{Sandbox, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;

    let vioux_service_impl = ViouxService::default();

    // Start gRPC server thread
    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(ViouxServer::new(vioux_service_impl))
            .serve(addr)
            .await
    });

    // Start UI thread
    App::run(Settings::default()).map_err(|e| e.into())
}

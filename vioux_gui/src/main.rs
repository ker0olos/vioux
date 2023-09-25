mod app;

use vioux::{ViouxServer, ViouxService};

use iced::{Sandbox, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;

    let vioux_service_impl = ViouxService::default();

    // start gRPC server on a new thread
    let grpc_server = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(ViouxServer::new(vioux_service_impl))
            .serve(addr)
            .await
    })
    .abort_handle();

    // start UI thread
    app::App::run(Settings::default()).map_err(|e| anyhow::anyhow!(e))?;
    // the code under this line won't execute until the UI is terminated

    // abort the grpc server
    grpc_server.abort();

    Ok(())
}

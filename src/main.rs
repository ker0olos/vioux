use vioux::{ViouxServer, ViouxService};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;

    let vioux_service_impl = ViouxService::default();

    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(ViouxServer::new(vioux_service_impl))
            .serve(addr)
            .await
    });

    loop {
        // TODO should be used as the ui thread later
        std::thread::yield_now();
    }

    // Ok(())
}

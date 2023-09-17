use vioux::{ViouxServer, ViouxService, FRAMES};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;

    let vioux_service_impl = ViouxService::default();

    let image = image::io::Reader::open("tests/assets/img.jpeg")
        .unwrap()
        .decode()
        .unwrap();

    let color_type = vioux::ColorType::from(image.color());

    // TODO REMOVE
    FRAMES.lock().unwrap().insert(
        0,
        vioux::Image {
            width: image.width(),
            height: image.height(),
            color_type: color_type.into(),
            data: image.into_bytes(),
        },
    );
    //

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

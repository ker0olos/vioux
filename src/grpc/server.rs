use image::{ImageBuffer, Pixel};
use tonic::{transport::Server, Request, Response, Status};

use super::proto::{
    vioux_server::{Vioux, ViouxServer},
    FrameRequestParameters, Image, SampleLayout,
};

#[derive(Default)]
struct ViouxService {}

fn as_sample_layout<P>(img: &ImageBuffer<P, Vec<P::Subpixel>>) -> SampleLayout
where
    P: Pixel + 'static,
{
    let channels = <P as Pixel>::CHANNEL_COUNT;

    let width = img.width();
    let height = img.height();

    let height_stride = (channels as usize)
        .checked_mul(width as usize)
        .expect("Row major packed image can not be described because it does not fit into memory");

    SampleLayout {
        channels: channels as u32,
        channel_stride: 1,
        width,
        width_stride: channels as u32,
        height,
        height_stride: height_stride as u32,
    }
}

#[tonic::async_trait]
impl Vioux for ViouxService {
    async fn request_frame(
        &self,
        _: Request<FrameRequestParameters>,
    ) -> Result<Response<Image>, Status> {
        // TODO
        let placeholder = image::open("img.jpeg").unwrap().into_rgb8();

        let reply = Image {
            sample_layout: Some(as_sample_layout(&placeholder)),
            raw: placeholder.into_raw(),
        };

        Ok(Response::new(reply))
    }
}

pub fn spawn() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;

    let timeline_service_impl = ViouxService::default();

    tokio::spawn(async move {
        Server::builder()
            .add_service(ViouxServer::new(timeline_service_impl))
            .serve(addr)
            .await
    });

    Ok(())
}

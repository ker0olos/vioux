use image::DynamicImage;
use tonic::{Code, Request, Response, Status};

use super::proto::{
    vioux_server::Vioux, Image, RequestFrameParameters, RequestedFrame, UpdateFrameParameters,
    UpdatedFrame,
};

#[derive(Default)]
pub struct ViouxService {}

#[tonic::async_trait]
impl Vioux for ViouxService {
    async fn request_frame(
        &self,
        _: Request<RequestFrameParameters>,
    ) -> Result<Response<RequestedFrame>, Status> {
        let placeholder = image::io::Reader::open("img.jpeg")
            .unwrap()
            .decode()
            .unwrap();

        let reply = RequestedFrame {
            image: Some(Image {
                channels: 4u32,
                height: placeholder.height(),
                width: placeholder.width(),
                raw: placeholder.into_rgba8().into_raw(),
            }),
        };

        Ok(Response::new(reply))
    }

    async fn update_frame(
        &self,
        params: Request<UpdateFrameParameters>,
    ) -> Result<Response<UpdatedFrame>, Status> {
        let Image {
            height,
            width,
            channels,
            raw,
        } = params.into_inner().image.unwrap();

        let img = if channels == 3 {
            DynamicImage::from(image::RgbImage::from_raw(width, height, raw).unwrap())
        } else if channels == 4 {
            DynamicImage::from(image::RgbaImage::from_raw(width, height, raw).unwrap())
        } else {
            return Err(Status::new(Code::InvalidArgument, "must be RGB or RGBA"));
        };

        img.save("img_export.jpeg").unwrap();

        Ok(Response::new(UpdatedFrame::default()))
    }
}

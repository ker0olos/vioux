use image::DynamicImage;
use tonic::{Request, Response, Status};

use super::proto::{self, vioux_server::Vioux, Image, RequestOpts, RequestedFrame, UpdatedFrame};

#[derive(Default)]
pub struct ViouxService {}

#[tonic::async_trait]
impl Vioux for ViouxService {
    async fn request_frame(
        &self,
        _request: Request<RequestOpts>,
    ) -> tonic::Result<Response<RequestedFrame>> {
        // TODO REMOVE placeholder
        let image = image::io::Reader::open("tests/assets/img.jpeg")
            .unwrap()
            .decode()
            .unwrap();

        let color_type: proto::ColorType = image.color().into();

        // send a raw decoded image to the client
        let response = RequestedFrame {
            image: Some(Image {
                width: image.width(),
                height: image.height(),
                raw: image.into_bytes(),
                color_type: color_type.into(),
            }),
        };

        Ok(Response::new(response))
    }

    async fn update_frame(
        &self,
        request: Request<RequestOpts>,
    ) -> tonic::Result<Response<UpdatedFrame>> {
        let image = request.into_inner().image.unwrap();

        // try to get a DynamicImage obj from the raw image
        let image = match image.color_type() {
            proto::ColorType::L8 => DynamicImage::from(
                image::GrayImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            proto::ColorType::La8 => DynamicImage::from(
                image::GrayAlphaImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            proto::ColorType::Rgb8 => DynamicImage::from(
                image::RgbImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            proto::ColorType::Rgba8 => DynamicImage::from(
                image::RgbaImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            color_type @ _ => {
                return Err(Status::unimplemented(format!(
                    "{} is not supported yet!",
                    color_type.as_str_name()
                )))
            }
        };

        // TODO REMOVE placeholder
        image.save("img_export.jpeg").unwrap();

        Ok(Response::new(UpdatedFrame::default()))
    }
}

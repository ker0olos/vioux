use image::DynamicImage;

use tonic::{Request, Response, Status};

use super::{
    image_types,
    proto::{
        vioux_server::Vioux, Audio, ColorType, Image, RequestOptions, RequestedAudio,
        RequestedFrame, UpdatedFrame,
    },
};

#[derive(Default)]
pub struct ViouxService {}

#[tonic::async_trait]
impl Vioux for ViouxService {
    async fn request_frame(
        &self,
        _request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedFrame>> {
        // TODO REMOVE placeholder
        let image = image::io::Reader::open("tests/assets/img.jpeg")
            .unwrap()
            .decode()
            .unwrap();

        let color_type = ColorType::from(image.color());

        // send a raw decoded image to the client
        Ok(Response::new(RequestedFrame {
            image: Some(Image {
                width: image.width(),
                height: image.height(),
                raw: image.into_bytes(),
                color_type: color_type.into(),
            }),
        }))
    }

    async fn update_frame(
        &self,
        request: Request<RequestOptions>,
    ) -> tonic::Result<Response<UpdatedFrame>> {
        let image = request.into_inner().image.unwrap();

        // try to get a DynamicImage obj from the raw image
        let _image = match image.color_type() {
            ColorType::L8 => DynamicImage::from(
                image_types::GrayImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            ColorType::La8 => DynamicImage::from(
                image_types::GrayAlphaImage::from_raw(image.width, image.height, image.raw)
                    .unwrap(),
            ),
            ColorType::Rgb8 => DynamicImage::from(
                image_types::RgbImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            ColorType::Rgba8 => DynamicImage::from(
                image_types::RgbaImage::from_raw(image.width, image.height, image.raw).unwrap(),
            ),
            // TODO #3
            // ColorType::L16 => DynamicImage::ImageLuma16(
            //     image_types::Gray16Image::from_raw(image.width, image.height, image.raw).unwrap(),
            // ),
            color_type => {
                return Err(Status::unimplemented(format!(
                    "{} is not supported yet!",
                    color_type.as_str_name()
                )))
            }
        };

        // image.save("img_export.jpeg").unwrap();

        Ok(Response::new(UpdatedFrame::default()))
    }

    async fn request_audio(
        &self,
        _request: Request<RequestOptions>,
    ) -> tonic::Result<Response<RequestedAudio>> {
        // TODO REMOVE placeholder
        let src = std::fs::File::open("tests/assets/sound.wav").unwrap();

        let result = Audio::from_media_source(Box::new(src));

        match result {
            // send a raw decoded audio to the client
            Ok(audio) => Ok(Response::new(RequestedAudio { audio: Some(audio) })),
            Err(err) => Err(Status::new(tonic::Code::Aborted, err.to_string())),
        }
    }
}

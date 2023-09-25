use std::path::PathBuf;

use md5::{Digest, Md5};
use vioux::{ColorType, Image, RequestOptions, Vioux, ViouxService};

fn compare_images(file_name: &str, requested_image: Image) {
    let image_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/assets/{}", file_name));

    let loaded_image = image::io::Reader::open(image_path)
        .unwrap()
        .decode()
        .unwrap();

    assert_eq!(requested_image.width, loaded_image.width());
    assert_eq!(requested_image.height, loaded_image.height());

    let requested_image = image::DynamicImage::from(
        image::RgbImage::from_raw(
            requested_image.width,
            requested_image.height,
            requested_image.data,
        )
        .unwrap(),
    );

    let first_hash = Md5::digest(requested_image.into_bytes());
    let second_hash = Md5::digest(loaded_image.into_bytes());

    assert_eq!(first_hash, second_hash);
}

#[tokio::test]
pub async fn test_request_frame() {
    let service = ViouxService::default();

    let response = service
        .request_frame(tonic::Request::new(RequestOptions {
            n: Some(0),
            layer: Some(0),
            image: None,
            audio: None,
        }))
        .await
        .unwrap();

    let image = response.into_inner().image.unwrap();

    compare_images("img.jpeg", image);
}

#[tokio::test]
pub async fn test_update_frame() {
    let service = ViouxService::default();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets/img.jpeg");

    let image = image::io::Reader::open(path).unwrap().decode().unwrap();

    let color_type = ColorType::from(image.color());

    // send a raw decoded image to the client
    let image = Some(Image {
        width: image.width(),
        height: image.height(),
        data: image.into_bytes(),
        color_type: color_type.into(),
        uuid: String::from(""),
        x: 0,
        y: 0,
    });

    let response = service
        .update_frame(tonic::Request::new(RequestOptions {
            image,
            n: Some(0),
            layer: Some(0),
            audio: None,
        }))
        .await;

    assert!(response.is_ok());
}

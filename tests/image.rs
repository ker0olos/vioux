use more_asserts::assert_gt;

use vioux::{ColorType, Image, RequestOpts, Vioux, ViouxService};

macro_rules! compare_images {
    ($title:literal => $file_name:literal => $image_one:expr) => {
        let snapshot_path =
            std::path::PathBuf::from(format!("tests/snapshots/{}_{}", $title, $file_name));

        std::fs::create_dir_all(snapshot_path.parent().unwrap())
            .expect("failed to create snapshot directory");

        if snapshot_path.as_path().exists() {
            let image_two = image::open(snapshot_path)
                .expect("failed to open image")
                .to_rgb8();

            let result = image_compare::rgb_similarity_structure(
                &image_compare::Algorithm::RootMeanSquared,
                $image_one,
                &image_two,
            )
            .expect("failed to compare images");

            assert_gt!(result.score, 0.98);
        } else {
            $image_one.save(snapshot_path).unwrap();

            panic!("new image was created for the first time!");
        }
    };
}

#[tokio::test]
pub async fn test_request_frame() {
    let service = ViouxService::default();

    let response = service
        .request_frame(tonic::Request::new(RequestOpts { image: None }))
        .await
        .unwrap();

    let image = response.into_inner().image.unwrap();

    let image = image::RgbImage::from_raw(image.width, image.height, image.raw).unwrap();

    compare_images!("request" => "img.jpeg" => &image);
}

#[tokio::test]
pub async fn test_update_frame() {
    let service = ViouxService::default();

    let image = image::io::Reader::open("tests/assets/img.jpeg")
        .unwrap()
        .decode()
        .unwrap();

    let color_type: ColorType = image.color().into();

    // send a raw decoded image to the client
    let image = Some(Image {
        width: image.width(),
        height: image.height(),
        raw: image.into_bytes(),
        color_type: color_type.into(),
    });

    let response = service
        .update_frame(tonic::Request::new(RequestOpts { image }))
        .await;

    assert_eq!(response.is_ok(), true);
}

use more_asserts::assert_gt;

use vioux::{RequestOpts, Vioux, ViouxService};

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
pub async fn test_request() {
    let service = ViouxService::default();

    let req = service
        .request_frame(tonic::Request::new(RequestOpts { image: None }))
        .await
        .unwrap();

    let image = req.into_inner().image.unwrap();

    let image = image::RgbImage::from_raw(image.width, image.height, image.raw).unwrap();

    compare_images!("request" => "img.jpeg" => &image);
}

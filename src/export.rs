use numpy::ndarray::Array3;

use std::path::PathBuf;

use video_rs::{Encoder, EncoderSettings, Locator, Time};

use super::store::FRAMES;

pub fn export_to_mp4() {
    video_rs::init().unwrap();

    let width = 512;
    let height = 512;
    let filename = "output.mp4";

    let settings = EncoderSettings::for_h264_yuv420p(width, height, false);

    let destination: Locator = PathBuf::from(filename).into();
    let mut encoder = Encoder::new(&destination, settings).expect("failed to create encoder");

    let duration: Time = Time::from_nth_of_a_second(24);

    let mut position = Time::zero();

    for image in FRAMES.lock().unwrap().values() {
        // let raw_pixels = image
        //     .clone()
        //     .to_dynamic_image()
        //     .unwrap()
        //     .to_rgb8()
        //     .into_raw();
        // let frame = Array3::from_shape_vec((height, width, 3), raw_pixels.clone()).unwrap();

        // TODO NOTE check color type
        // TODO not all frames will have the canvas width and height

        let frame = Array3::from_shape_vec((height, width, 3), image.data.clone()).unwrap();

        encoder
            .encode(&frame.into(), &position)
            .expect("failed to encode frame");

        // Update the current position
        position = position.aligned_with(&duration).add();
    }

    encoder.finish().expect("failed to finish encoder");
}

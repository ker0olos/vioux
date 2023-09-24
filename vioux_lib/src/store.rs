use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use super::grpc::proto::{Audio, Image};

lazy_static::lazy_static! {
    pub static ref FRAMES: Mutex<HashMap<u64, Image>> = {
        // TODO REMOVE
        let mut m = HashMap::new();

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets/img.jpeg");

        let image = image::io::Reader::open(path)
            .unwrap()
            .decode()
            .unwrap();

        let color_type = super::ColorType::from(image.color());

        m.insert(
            0,
            super::Image {
                width: image.width(),
                height: image.height(),
                color_type: color_type.into(),
                data: image.into_bytes(),
                x: 0,
                y: 0
            },
        );

        Mutex::new(m)
    };

    pub static ref SEGMENTS: Mutex<HashMap<u64, Audio>> = {
        // TODO REMOVE
        let mut m = HashMap::new();

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets/sound.wav");

        let src = std::fs::File::open(path)
            .unwrap();

        m.insert(
            0,
            Audio::from_media_source(Box::new(src)).unwrap()
        );

        Mutex::new(m)
    };
}

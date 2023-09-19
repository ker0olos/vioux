use std::{collections::HashMap, sync::Mutex};

use super::grpc::proto::{Audio, Image};

lazy_static::lazy_static! {
    pub static ref FRAMES: Mutex<HashMap<u64, Image>> = {
        // TODO REMOVE
        let mut m = HashMap::new();

        let image = image::io::Reader::open("tests/assets/img.jpeg")
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
            },
        );

        Mutex::new(m)
    };

    pub static ref SEGMENTS: Mutex<HashMap<u64, Audio>> = {
        // TODO REMOVE
        let mut m = HashMap::new();

        let src = std::fs::File::open("tests/assets/sound.wav")
            .unwrap();

        m.insert(
            0,
            Audio::from_media_source(Box::new(src)).unwrap()
        );

        Mutex::new(m)
    };
}

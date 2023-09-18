use std::{collections::HashMap, sync::Mutex};

use super::grpc::proto::Image;

lazy_static::lazy_static! {
    pub(crate) static ref FRAMES: Mutex<HashMap<u32, Image>> = {
        // TODO REMOVE test frame
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
        //

        Mutex::new(m)
    };
}

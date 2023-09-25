use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use uuid::Uuid;

use crate::grpc::proto::{Audio, Image};

pub fn get_frames_by_layer(l: u32) -> Vec<Image> {
    // TODO remove debug code
    let layers = FRAMES_LAYERS.lock().unwrap();
    if layers.len() == 0 {
        drop(layers);

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets/img.jpeg");

        let image = image::io::Reader::open(path).unwrap().decode().unwrap();

        let color_type = super::ColorType::from(image.color());

        insert_frame_to_layer(
            0,
            Image {
                uuid: Uuid::new_v4().to_string(),
                width: image.width(),
                height: image.height(),
                color_type: color_type.into(),
                data: image.into_bytes(),
                x: 0,
                y: 0,
            },
        );
    } else {
        drop(layers);
    }
    // TODO

    let mut layers = FRAMES_LAYERS.lock().unwrap();

    let frames_by_uuid = FRAMES_BY_UUID.lock().unwrap();

    let layer = layers.entry(l).or_insert(vec![]);

    let frames = layer
        .iter()
        .map(|uuid| frames_by_uuid.get(uuid).expect("uuid not found").clone())
        .collect();

    drop(layers);
    drop(frames_by_uuid);

    frames
}

pub fn insert_frame_to_layer(l: u32, image: Image) -> String {
    let mut layers = FRAMES_LAYERS.lock().unwrap();

    let mut frames_by_uuid = FRAMES_BY_UUID.lock().unwrap();

    let layer = layers.entry(l).or_insert(vec![]);

    let uuid = image.uuid.clone();

    frames_by_uuid.insert(uuid.clone(), image);

    layer.push(uuid.clone());

    drop(layers);
    drop(frames_by_uuid);

    uuid
}

pub fn update_frame(image: Image) -> anyhow::Result<()> {
    let mut frames_by_uuid = FRAMES_BY_UUID.lock().unwrap();

    if frames_by_uuid.contains_key(&image.uuid) {
        frames_by_uuid.insert(image.uuid.clone(), image);
        drop(frames_by_uuid);
        Ok(())
    } else {
        Err(anyhow::anyhow!("Uuid not found"))
    }
}

lazy_static::lazy_static! {
    static ref FRAMES_LAYERS: Mutex<HashMap<u32, Vec<String>>> = Mutex::new(HashMap::new());
    static ref FRAMES_BY_UUID: Mutex<HashMap<String, Image>> = Mutex::new(HashMap::new());

    pub static ref SEGMENTS: Mutex<HashMap<u64, Audio>> = {
        // TODO remove debug code
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

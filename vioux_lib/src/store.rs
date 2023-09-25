use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use uuid::Uuid;

use crate::grpc::proto::{Audio, Image};

pub fn get_frames_by_layer(l: u32) -> Vec<Image> {
    let layers = FRAMES_LAYERS.lock().unwrap();

    // TODO remove debug code
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
    // debug code

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

pub fn get_audio_by_layer(l: u32) -> Vec<Audio> {
    let layers = AUDIO_LAYERS.lock().unwrap();

    // TODO remove debug code
    if layers.len() == 0 {
        drop(layers);

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/assets/sound.wav");

        let src = std::fs::File::open(path).unwrap();

        insert_audio_to_layer(0, Audio::from_media_source(Box::new(src)).unwrap());
    } else {
        drop(layers);
    }
    // debug code

    let mut layers = AUDIO_LAYERS.lock().unwrap();

    let audio_by_uuid = AUDIO_BY_UUID.lock().unwrap();

    let layer = layers.entry(l).or_insert(vec![]);

    let audio = layer
        .iter()
        .map(|uuid| audio_by_uuid.get(uuid).expect("uuid not found").clone())
        .collect();

    drop(layers);
    drop(audio_by_uuid);

    audio
}

pub fn insert_audio_to_layer(l: u32, audio: Audio) -> String {
    let mut layers = AUDIO_LAYERS.lock().unwrap();

    let mut audio_by_uuid = AUDIO_BY_UUID.lock().unwrap();

    let layer = layers.entry(l).or_insert(vec![]);

    let uuid = audio.uuid.clone();

    audio_by_uuid.insert(uuid.clone(), audio);

    layer.push(uuid.clone());

    drop(layers);
    drop(audio_by_uuid);

    uuid
}

pub fn update_audio(audio: Audio) -> anyhow::Result<()> {
    let mut audio_by_uuid = AUDIO_BY_UUID.lock().unwrap();

    if audio_by_uuid.contains_key(&audio.uuid) {
        audio_by_uuid.insert(audio.uuid.clone(), audio);
        drop(audio_by_uuid);
        Ok(())
    } else {
        Err(anyhow::anyhow!("Uuid not found"))
    }
}

lazy_static::lazy_static! {
    static ref FRAMES_LAYERS: Mutex<HashMap<u32, Vec<String>>> = Mutex::new(HashMap::new());
    static ref FRAMES_BY_UUID: Mutex<HashMap<String, Image>> = Mutex::new(HashMap::new());

    static ref AUDIO_LAYERS: Mutex<HashMap<u32, Vec<String>>> = Mutex::new(HashMap::new());
    static ref AUDIO_BY_UUID: Mutex<HashMap<String, Audio>> = Mutex::new(HashMap::new());
}

use std::path::PathBuf;

use md5::{Digest, Md5};

use vioux::{Audio, RequestOptions, Vioux, ViouxService};

fn compare_clips(file_name: &str, requested_clip: Audio) {
    let clip_path = PathBuf::from(format!("tests/assets/{}", file_name));

    let src = std::fs::File::open(clip_path).unwrap();

    let loaded_clip =
        Audio::from_media_source(Box::new(src)).expect("failed to decode original file");

    assert_eq!(requested_clip.sample_rate, loaded_clip.sample_rate);
    assert_eq!(requested_clip.sample_width, loaded_clip.sample_width);
    assert_eq!(requested_clip.channels, loaded_clip.channels);

    let first_hash = Md5::digest(requested_clip.data);
    let second_hash = Md5::digest(loaded_clip.data);

    assert_eq!(first_hash, second_hash);
}

#[tokio::test]
pub async fn test_request_audio() {
    let service = ViouxService::default();

    let response = service
        .request_audio(tonic::Request::new(RequestOptions::default()))
        .await
        .unwrap();

    let audio = response.into_inner().audio.unwrap();

    compare_clips("sound.wav", audio);
}

#[tokio::test]
pub async fn test_update_audio() {
    let service = ViouxService::default();

    let src = std::fs::File::open("tests/assets/sound.wav").unwrap();

    let audio = Audio::from_media_source(Box::new(src)).expect("failed to decode original file");
    let audio = Some(audio);

    let response = service
        .update_audio(tonic::Request::new(RequestOptions { audio, image: None }))
        .await;

    assert!(response.is_ok());
}

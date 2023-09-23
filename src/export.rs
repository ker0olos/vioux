use numpy::ndarray::Array3;

use std::{io::Write, path::PathBuf};

use video_rs::{Encoder, EncoderSettings, Time};

use super::store::{FRAMES, SEGMENTS};

pub(crate) fn export_to_mp3() -> Result<(), anyhow::Error> {
    let segments = SEGMENTS.lock().unwrap();

    let seg = segments.get(&0).unwrap();

    // TODO use ffmpeg-next
    let mut child = std::process::Command::new("ffmpeg")
        // format
        .arg("-f")
        .arg(format!("{}", &seg.codec[4..]))
        // codec
        .arg("-acodec")
        .arg(format!("{}", &seg.codec))
        //  sample rate
        .arg("-ar")
        .arg(format!("{}", &seg.sample_rate))
        // channels
        .arg("-ac")
        .arg(format!("{}", &seg.channels))
        //
        .arg("-i")
        .arg("-") // input data from stdin
        .arg("-f")
        .arg("wav") // output format
        .arg("output.wav") // output filename
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start ffmpeg");

    // Write the raw audio data to ffmpeg's stdin
    child.stdin.as_mut().unwrap().write_all(&seg.data).unwrap();

    // Wait for ffmpeg to finish
    child.wait().unwrap();

    Ok(())
}

pub(crate) fn add_audio_to_video() {
    // TODO untested
    // TODO use ffmpeg-next
    let mut child = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg("video.mp4")
        // sets an offset of 30 seconds for the next input file.
        // This means the audio will start playing nth seconds into the video.
        .arg("-itsoffset")
        .arg("-00:00:30")
        //
        .arg("-i")
        .arg("audio.mp3")
        // [0:a][1:a] selects the audio streams from the first and second input files
        // [a] the name of the output audio stream
        .arg("-filter_complex")
        .arg("\"[0:a][1:a]amerge=inputs=2[a]\"")
        .arg("-map 0:v -map \"[a]\"")
        // ensures that the video stream is copied without re-encoding
        .arg("-c:v")
        .arg("copy")
        // stop encoding when the shortest input stream ends
        .arg("-shortest")
        //
        .arg("output.mp4")
        .spawn()
        .expect("Failed to start ffmpeg");

    // Wait for ffmpeg to finish
    child.wait().unwrap();
}

pub(crate) fn export_to_mp4() {
    video_rs::init().expect("falied to initialize video-rs");

    let canvas_width = 512;
    let canvas_height = 512;

    let filename = "output.mp4";

    let settings = EncoderSettings::for_h264_yuv420p(canvas_width, canvas_height, false);

    let destination = PathBuf::from(filename);

    let mut encoder =
        Encoder::new(&destination.into(), settings).expect("failed to create video encoder");

    let frames_per_second = 30;

    let frame_duration: Time = Time::from_nth_of_a_second(frames_per_second);

    let mut insert_timestamp = Time::zero();

    for image in FRAMES.lock().unwrap().values() {
        let img = image
            .clone()
            .to_dynamic_image()
            .expect("failed to convert image to dynamic image");

        let frame =
            Array3::from_shape_vec((canvas_height, canvas_width, 3), img.to_rgb8().into_raw())
                .expect("failed to convert dynamic image to ndarray");

        encoder
            .encode(&frame.into(), &insert_timestamp)
            .expect("failed to encode frame");

        // add the frame duration to the timestamp
        insert_timestamp = insert_timestamp.aligned_with(&frame_duration).add();
    }

    encoder.finish().expect("failed to finish encode video");
}

pub mod client;
pub mod server;
mod utils;

#[allow(clippy::all)]
pub mod proto {
    tonic::include_proto!("_");
}

impl Into<proto::ColorType> for image::ColorType {
    fn into(self) -> proto::ColorType {
        match self {
            image::ColorType::L8 => proto::ColorType::L8,
            image::ColorType::La8 => proto::ColorType::La8,
            image::ColorType::Rgb8 => proto::ColorType::Rgb8,
            image::ColorType::Rgba8 => proto::ColorType::Rgba8,
            image::ColorType::L16 => proto::ColorType::L16,
            image::ColorType::La16 => proto::ColorType::La16,
            image::ColorType::Rgb16 => proto::ColorType::Rgb16,
            image::ColorType::Rgba16 => proto::ColorType::Rgba16,
            image::ColorType::Rgb32F => proto::ColorType::Rgb32F,
            image::ColorType::Rgba32F => proto::ColorType::Rgba32F,
            _ => unreachable!(),
        }
    }
}

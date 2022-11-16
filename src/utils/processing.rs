use numpy::ndarray::{Array3, Dim, ShapeBuilder};

use crate::grpc::proto::Image;

pub(crate) fn into_ndarray3(image: Image) -> numpy::ndarray::Array<u8, Dim<[usize; 3]>> {
    let channel_stride = 1u32;
    let width_stride = image.channels;

    let height_stride = (image.channels as usize)
        .checked_mul(image.width as usize)
        .expect("Row major packed image can not be described because it does not fit into memory");

    let shape = (
        image.height as usize,
        image.width as usize,
        image.channels as usize,
    );

    let strides = (
        height_stride as usize,
        width_stride as usize,
        channel_stride as usize,
    );

    Array3::from_shape_vec(shape.strides(strides), image.raw).unwrap()
}

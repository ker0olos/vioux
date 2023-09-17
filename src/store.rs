use std::{collections::HashMap, sync::Mutex};

use super::grpc::proto::Image;

lazy_static::lazy_static! {
    pub static ref FRAMES: Mutex<HashMap<u32, Image>> = Mutex::new(HashMap::new());
}

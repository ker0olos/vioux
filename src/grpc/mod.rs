pub mod client;
pub mod server;

#[allow(clippy::all)]
pub(crate) mod proto {
    tonic::include_proto!("_");
}

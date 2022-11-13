pub(crate) mod client;
pub mod server;

#[allow(clippy::all)]
pub(crate) mod placeholder {
    tonic::include_proto!("placeholder");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("placeholder_descriptor");
}

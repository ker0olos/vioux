fn main() -> std::io::Result<()> {
    tonic_build::configure().compile(&["proto/main.proto"], &["proto"])?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    tonic_build::configure().compile(&["proto/vioux.proto"], &["proto"])?;

    Ok(())
}

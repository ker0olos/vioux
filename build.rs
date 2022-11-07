fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/schema/placeholder.proto"], &["src/"])
}

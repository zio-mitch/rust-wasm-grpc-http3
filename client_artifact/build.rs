fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = manifest_dir.join("../proto");
    let proto_file = proto_dir.join("ping.proto");

    println!("cargo:rerun-if-changed={}", proto_file.display());

    tonic_build::configure()
        .build_transport(false)
        .compile_protos(
            &[proto_file],
            &[proto_dir]
        )
        .map_err(|e| {
            eprintln!("ERRORE PROTOC: {}", e);
            e
        })?;

    Ok(())
}
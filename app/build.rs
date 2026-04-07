fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_transport(false) // Fondamentale per WASM
        .compile_protos(
            &["../proto/ping.proto"], // Primo argomento: slice di path ai file
            &["../proto"]             // Secondo argomento: slice di cartelle di inclusione
        )
        .map_err(|e| {
            eprintln!("ERRORE PROTOC: {}", e);
            e
        })?;

    Ok(())
}
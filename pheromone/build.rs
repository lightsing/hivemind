fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(cfg!(feature = "bee-client"))
        .build_server(cfg!(feature = "bee-server"))
        .compile(&["proto/bee.proto"], &["proto"])?;
    tonic_build::configure()
        .build_client(cfg!(feature = "queen-client"))
        .build_server(cfg!(feature = "queen-server"))
        .compile(&["proto/queen.proto"], &["proto"])?;
    tonic_build::configure()
        .build_client(cfg!(feature = "hive-client"))
        .build_server(cfg!(feature = "hive-server"))
        .compile(&["proto/hive.proto"], &["proto"])?;
    Ok(())
}

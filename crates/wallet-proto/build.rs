fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "proto/wallet/v1/common.proto",
        "proto/wallet/v1/user.proto",
        "proto/wallet/v1/token.proto",
        "proto/wallet/v1/transaction.proto",
        "proto/wallet/v1/network.proto",
        "proto/wallet/v1/swap.proto",
        "proto/wallet/v1/gaspool.proto",
        "proto/wallet/v1/market.proto",
        "proto/wallet/v1/dapp.proto",
        "proto/wallet/v1/rent.proto",
        "proto/wallet/v1/solana.proto",
        "proto/wallet/v1/cms.proto",
        "proto/wallet/v1/admin/admin.proto",
    ];

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let root = manifest_dir.join("../..");
    let proto_files: Vec<_> = protos.iter().map(|p| root.join(p)).collect();
    let include = root.join("proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&proto_files, &[include])?;

    Ok(())
}

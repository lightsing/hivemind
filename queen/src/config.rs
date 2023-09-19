use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Queen configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub queen_grpc: GrpcServerConfig,
    pub hive_grpc: GrpcServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcServerConfig {
    pub addr: SocketAddr,
}

impl Config {
    pub fn load() -> Self {
        toml::from_str(
            &std::fs::read_to_string(
                std::env::var("QUEEN_CONFIG").unwrap_or_else(|_| "config.toml".to_string()),
            )
            .unwrap(),
        )
        .unwrap()
    }
}

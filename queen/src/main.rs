#[macro_use]
extern crate tracing;

use crate::config::Config;
use crate::scheduler::aws::AwsScheduler;
use aws_config;
use pheromone::{async_trait, GrpcRequest, GrpcResponse, GrpcServer, GrpcStatus};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::EnvFilter;

mod config;
mod scheduler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let config = Arc::new(RwLock::new(Config::load()));
    AwsScheduler::new().await;
}

async fn grpc_server(config: Arc<RwLock<Config>>) {}

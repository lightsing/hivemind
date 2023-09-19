use bollard::Docker;
use pheromone::{async_trait, Bee, BeeServer, GrpcRequest, GrpcResponse, GrpcServer, GrpcStatus};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let docker = Docker::connect_with_socket_defaults()?;

    let mut join_set = JoinSet::new();
    let _ = join_set.spawn(test_host_availability(docker.clone()));
    let _ = join_set.spawn(grpc_server(docker.clone()));

    if let Some(e) = join_set.join_next().await {
        e??;
    }
    unreachable!()
}

async fn test_host_availability(docker: Docker) -> anyhow::Result<()> {
    loop {
        docker.ping().await?;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

struct BeeImpl {
    docker: Docker,
}

#[async_trait]
impl Bee for BeeImpl {
    async fn health_check(&self, _: GrpcRequest<()>) -> Result<GrpcResponse<()>, GrpcStatus> {
        Ok(GrpcResponse::new(()))
    }
}

impl BeeImpl {
    fn new(docker: Docker) -> Self {
        Self { docker }
    }
}

async fn grpc_server(docker: Docker) -> anyhow::Result<()> {
    let addr = "[::]:50051".parse().unwrap();
    GrpcServer::builder()
        .add_service(BeeServer::new(BeeImpl::new(docker)))
        .serve(addr)
        .await?;
    Ok(())
}

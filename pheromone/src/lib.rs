pub use prost;
pub use tonic;
pub use tonic::{
    async_trait, transport::Server as GrpcServer, Request as GrpcRequest, Response as GrpcResponse,
    Status as GrpcStatus,
};

mod bee {
    tonic::include_proto!("bee");
}

mod queen {
    tonic::include_proto!("queen");
}

mod hive {
    tonic::include_proto!("hive");
}

#[cfg(feature = "bee-client")]
pub use bee::bee_client::BeeClient;
#[cfg(feature = "bee-server")]
pub use bee::bee_server::{Bee, BeeServer};
#[cfg(feature = "hive-client")]
pub use hive::hive_client::HiveClient;
#[cfg(feature = "hive-server")]
pub use hive::hive_server::{Hive, HiveServer};
#[cfg(feature = "queen-client")]
pub use queen::queen_client::QueenClient;
#[cfg(feature = "queen-server")]
pub use queen::queen_server::{Queen, QueenServer};

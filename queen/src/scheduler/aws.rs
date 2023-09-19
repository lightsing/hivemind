use aws_config::{load_from_env, SdkConfig};

pub struct AwsScheduler {
    config: SdkConfig,
}

impl AwsScheduler {
    pub async fn new() -> Self {
        let config = load_from_env().await;
        Self { config }
    }
}

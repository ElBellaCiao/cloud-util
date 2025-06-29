mod ssm_parameter_client;

pub use ssm_parameter_client::SsmParameterClient;

#[async_trait::async_trait]
pub trait ConfigStore: Send + Sync {
    async fn get_parameter(&self, key: &str) -> anyhow::Result<String>;
}

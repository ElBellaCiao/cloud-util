use crate::InstanceId;
use anyhow::Result;

mod ssm;
pub use ssm::SsmClient;

#[async_trait::async_trait]
pub trait Manager: Send + Sync {
    async fn send_and_wait(&self, instance_ids: &[InstanceId], commands: Vec<String>)
    -> Result<()>;
    async fn send(&self, instance_ids: &[InstanceId], commands: Vec<String>) -> Result<()>;
}

use anyhow::Result;

mod ssm;

pub use ssm::Ssm;

#[async_trait::async_trait]
pub trait Manager: Send + Sync {
    async fn send(&self, instance_ids: &[String], commands: Vec<String>) -> Result<()>;
}
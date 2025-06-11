use anyhow::Result;

mod ec2_metadata;

pub use ec2_metadata::Ec2Metadata;

#[async_trait::async_trait]
pub trait Metadata: Send + Sync {
    async fn get_self_id(&self) -> Result<String>;
}

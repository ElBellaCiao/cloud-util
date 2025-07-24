use anyhow::Result;

mod ec2_metadata_client;

pub use ec2_metadata_client::Ec2MetadataClient;

#[async_trait::async_trait]
pub trait Metadata: Send + Sync {
    async fn get_self_id(&self) -> Result<String>;
    async fn get_tag_value(&self, tag_key: &str) -> Result<String>;
    async fn get_private_ip(&self) -> Result<String>;
}

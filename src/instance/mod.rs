use anyhow::Result;
use std::collections::HashMap;

mod ec2_client;
mod instance_id;
mod instance_model;

pub use ec2_client::Ec2Client;
pub use instance_id::InstanceId;
pub use instance_model::{InstanceMetadata, InstanceState};

#[async_trait::async_trait]
pub trait Instance: Send + Sync {
    async fn get_tags_by_instance(
        &self,
        instance_id: &InstanceId,
    ) -> Result<HashMap<String, String>>;
    async fn get_instances_by_tags(
        &self,
        tags: &HashMap<String, String>,
    ) -> Result<Vec<InstanceId>>;
    async fn start_instances(&self, instance_ids: &[InstanceId]) -> Result<()>;
    async fn stop_instances(&self, instance_ids: &[InstanceId]) -> Result<()>;
    async fn get_instance_metadata(&self, instance_id: &InstanceId) -> Result<InstanceMetadata>;
}

use std::collections::HashMap;
use anyhow::Result;

pub mod ec2;
pub use ec2::Ec2;

#[async_trait::async_trait]
pub trait Instance: Send + Sync {
    async fn get_tags_by_instance(&self, instance_id: &str) -> Result<HashMap<String, String>>;
    async fn get_instances_by_tags(&self, tags: &HashMap<String, String>) -> Result<Vec<String>>;
    async fn start_instances(&self, instance_ids: &[String]) -> Result<()>;
    async fn stop_instances(&self, instance_ids: &[String]) -> Result<()>;
}

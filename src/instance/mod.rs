use crate::common::CloudError;
use anyhow::Result;
use std::collections::HashMap;
use std::net::IpAddr;

pub mod ec2;
pub mod instance_id;

pub use ec2::Ec2;
pub use instance_id::InstanceId;

pub struct InstanceMetadata {
    pub private_ip: IpAddr,
}

#[async_trait::async_trait]
pub trait Instance: Send + Sync {
    async fn get_tags_by_instance(&self, instance_id: &InstanceId) -> Result<HashMap<String, String>, CloudError>;
    async fn get_instances_by_tags(&self, tags: &HashMap<String, String>) -> Result<Vec<InstanceId>>;
    async fn start_instances(&self, instance_ids: &[String]) -> Result<()>;
    async fn stop_instances(&self, instance_ids: &[String]) -> Result<()>;
    async fn get_instance_metadata(&self, instance_id: &InstanceId) -> Result<InstanceMetadata, CloudError>;
}

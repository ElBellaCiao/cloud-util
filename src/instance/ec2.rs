use crate::helper::aws_client_or_default;
use anyhow::{anyhow, Result};
use aws_sdk_ec2::client::Waiters;
use aws_sdk_ec2::types::Filter;
use aws_sdk_ec2::Client;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use crate::instance::InstanceMetadata;

pub struct Ec2 {
    client: Client,
}

impl Ec2 {
    pub async fn new(client: Option<Client>) -> Self {
        let client = aws_client_or_default(client, Client::new).await;
        Self { client }
    }
}

#[async_trait::async_trait]
impl crate::instance::Instance for Ec2 {
    async fn get_tags_by_instance(&self, instance_id: &str) -> Result<HashMap<String, String>> {
        let filters = Filter::builder()
            .name("resource-id")
            .values(instance_id)
            .build();
        let response = self.client
            .describe_tags()
            .filters(filters)
            .send()
            .await?;
        let tag_map = response.tags().iter()
            .filter_map(|tag| Some((tag.key()?.to_string(), tag.value()?.to_string())))
            .collect();

        println!("Found \"{:?}\" tag(s) from {}", tag_map, instance_id);
        Ok(tag_map)
    }

    async fn get_instances_by_tags(&self, tags: &HashMap<String, String>) -> Result<Vec<String>> {
        let filters = tags.iter().map(|(key, value)| {
            Filter::builder()
                .name(format!("tag:{}", key))
                .values(value)
                .build()
        }).collect();

        let response = self.client
            .describe_instances()
            .set_filters(Some(filters))
            .send()
            .await?;

        let instance_ids: Vec<String> = response.reservations()
            .iter()
            .flat_map(|reservation| { reservation.instances() })
            .filter_map(|instance| { instance.instance_id() })
            .map(|instance_id| instance_id.to_string())
            .collect();

        println!("Found {} instance(s) with tags \"{:?}\"", instance_ids.len(), tags);
        Ok(instance_ids)
    }

    async fn start_instances(&self, instance_ids: &[String]) -> Result<()> {
        self.client
            .start_instances()
            .set_instance_ids(Some(instance_ids.to_vec()))
            .send()
            .await?;

        self.client
            .wait_until_instance_status_ok()
            .set_instance_ids(Some(instance_ids.to_vec()))
            .wait(Duration::from_secs(6000))
            .await?;

        println!("Started {} instance(s)", instance_ids.len());
        Ok(())
    }

    async fn stop_instances(&self, instance_ids: &[String]) -> Result<()> {
        self.client
            .stop_instances()
            .set_instance_ids(Some(instance_ids.to_vec()))
            .send()
            .await?;

        self.client
            .wait_until_instance_stopped()
            .set_instance_ids(Some(instance_ids.to_vec()))
            .wait(Duration::from_secs(6000))
            .await?;

        println!("Started {} instance(s)", instance_ids.len());
        Ok(())
    }

    async fn get_instance_metadata(&self, instance_id: &str) -> Result<InstanceMetadata> {
        let response = self.client
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await?;

        let instance = response
            .reservations()
            .iter()
            .flat_map(|r| r.instances())
            .find(|inst| inst.instance_id() == Some(instance_id))
            .ok_or_else(|| anyhow!("Instance {} not found", instance_id))?;

        let private_ip = instance.private_ip_address()
            .ok_or_else(|| anyhow::anyhow!("No private IP found for {}", instance_id))?
            .parse::<IpAddr>()?;

        let metadata = InstanceMetadata {
            private_ip
        };

        Ok(metadata)
    }
}

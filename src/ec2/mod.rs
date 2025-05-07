use std::collections::HashMap;
use std::time::Duration;
use aws_sdk_ec2::client::Waiters;
use anyhow::Result;
use aws_sdk_ec2::types::Filter;
use aws_sdk_ec2::Client;
use crate::helper::aws_client_or_default;

pub struct Ec2 {
    client: Client,
}

impl Ec2 {
    pub async fn new(client: Option<Client>) -> Self {
        let client = aws_client_or_default(client, Client::new).await;
        Self { client }
    }

    pub async fn get_tags_by_instance(&self, instance_id: &String) -> Result<HashMap<String, String>> {
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

    pub async fn get_instances_by_tags(&self, tags: &HashMap<String, String>) -> Result<Vec<String>> {
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

    pub async fn start_instances(&self, instance_ids: &[String]) -> Result<()> {
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

    pub async fn stop_instances(&self, instance_ids: &[String]) -> Result<()> {
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
}
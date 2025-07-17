use crate::InstanceId;
use crate::instance::InstanceMetadata;
use anyhow::{Result, anyhow};
use aws_sdk_ec2::Client;
use aws_sdk_ec2::client::Waiters;
use aws_sdk_ec2::types::{Filter, Instance};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use tracing::info;

pub struct Ec2Client {
    client: Client,
}

impl Ec2Client {
    pub fn builder() -> Ec2ClientBuilder {
        Ec2ClientBuilder::default()
    }

    async fn find_instances_by_filter(
        &self,
        filters: Vec<Filter>,
    ) -> Result<Vec<InstanceMetadata>> {
        let response = self
            .client
            .describe_instances()
            .set_filters(Some(filters.clone()))
            .send()
            .await?;

        let instance_ids = response
            .reservations()
            .iter()
            .flat_map(|reservation| reservation.instances())
            .map(|instance| instance.try_into())
            .collect::<Result<Vec<InstanceMetadata>, _>>()?;

        info!(
            "Found \"{:?}\" with filters \"{:?}\"",
            instance_ids.clone(),
            filters
        );

        Ok(instance_ids)
    }
}

#[async_trait::async_trait]
impl crate::instance::Instance for Ec2Client {
    async fn get_tags_by_instance(
        &self,
        instance_id: &InstanceId,
    ) -> Result<HashMap<String, String>> {
        let filters = Filter::builder()
            .name("resource-id")
            .values(instance_id.as_ref())
            .build();
        let response = self.client.describe_tags().filters(filters).send().await?;
        let tag_map = response
            .tags()
            .iter()
            .filter_map(|tag| Some((tag.key()?.to_string(), tag.value()?.to_string())))
            .collect();

        info!("Found \"{tag_map:?}\" tag(s) from {instance_id}");
        Ok(tag_map)
    }

    async fn get_instances_by_tags(
        &self,
        tags: &HashMap<String, String>,
    ) -> Result<Vec<InstanceMetadata>> {
        let filters = tags
            .iter()
            .map(|(key, value)| {
                Filter::builder()
                    .name(format!("tag:{}", key))
                    .values(value)
                    .build()
            })
            .collect();

        self.find_instances_by_filter(filters).await
    }

    async fn start_instances(&self, instance_ids: &[InstanceId]) -> Result<()> {
        let instance_id_strings: Vec<String> = instance_ids
            .iter()
            .map(AsRef::as_ref)
            .map(str::to_string)
            .collect();

        self.client
            .start_instances()
            .set_instance_ids(Some(instance_id_strings.clone()))
            .send()
            .await?;

        self.client
            .wait_until_instance_status_ok()
            .set_instance_ids(Some(instance_id_strings.clone()))
            .wait(Duration::from_secs(6000))
            .await?;

        info!("Started {} instance(s)", instance_ids.len());
        Ok(())
    }

    async fn stop_instances(&self, instance_ids: &[InstanceId]) -> Result<()> {
        let instance_id_strings: Vec<String> = instance_ids
            .iter()
            .map(AsRef::as_ref)
            .map(str::to_string)
            .collect();

        self.client
            .stop_instances()
            .set_instance_ids(Some(instance_id_strings.clone()))
            .send()
            .await?;

        self.client
            .wait_until_instance_stopped()
            .set_instance_ids(Some(instance_id_strings.clone()))
            .wait(Duration::from_secs(6000))
            .await?;

        info!("Stopped {} instance(s)", instance_ids.len());
        Ok(())
    }
}

impl TryFrom<&Instance> for InstanceMetadata {
    type Error = anyhow::Error;

    fn try_from(value: &Instance) -> std::result::Result<Self, Self::Error> {
        let instance_id = value
            .instance_id
            .clone()
            .ok_or(anyhow!("No instance id Found"))?;

        let private_ip = value
            .private_ip_address()
            .ok_or(anyhow!("No private IP found for {}", instance_id))?
            .parse::<IpAddr>()?;

        let status = value
            .state()
            .and_then(|state| state.name())
            .ok_or(anyhow!("No status found for {}", instance_id))?;

        let tags = value
            .tags()
            .iter()
            .filter_map(|tag| match (tag.key(), tag.value()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            })
            .collect();

        Ok(InstanceMetadata {
            private_ip,
            instance_id: InstanceId::new(instance_id)?,
            status: status.into(),
            tags,
        })
    }
}

#[derive(Default)]
pub struct Ec2ClientBuilder {
    client: Option<Client>,
}

impl Ec2ClientBuilder {
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn build(self) -> Ec2Client {
        let client = match self.client {
            Some(client) => client,
            None => {
                let config = aws_config::load_from_env().await;
                Client::new(&config)
            }
        };

        Ec2Client { client }
    }
}

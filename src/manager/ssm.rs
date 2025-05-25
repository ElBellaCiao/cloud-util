use std::time::Duration;
use aws_sdk_ssm::client::Waiters;
use anyhow::{anyhow, bail, Result};
use aws_sdk_ssm::Client;
use tracing::{info, warn};
use crate::helper::aws_client_or_default;
use crate::InstanceId;

pub struct Ssm {
    client: Client,
}

impl Ssm {
    const WORKING_DIR: &'static str = "/home/ec2-user";

    pub async fn new(client: Option<Client>) -> Self {
        let client = aws_client_or_default(client, Client::new).await;
        Self { client }
    }
}

#[async_trait::async_trait]
impl crate::manager::Manager for Ssm {
    async fn send(&self, instance_ids: &[InstanceId], commands: Vec<String>) -> Result<()> {
        info!("Preparing commands: {commands:?}");

        let instance_id_strings: Vec<String> = instance_ids
            .iter()
            .map(AsRef::as_ref)
            .map(str::to_string)
            .collect();

        let response = self.client.send_command()
            .set_instance_ids(Some(instance_id_strings.clone()))
            .document_name("AWS-RunShellScript")
            .parameters("commands", commands)
            .parameters("workingDirectory", vec![Self::WORKING_DIR.to_string()])
            .send()
            .await?;

        let command = response.command().ok_or_else(|| anyhow!("missing command in response"))?;
        let command_id = command.command_id().ok_or_else(|| anyhow!("missing command id in response"))?;

        for instance_id in &instance_id_strings {
            info!("Sending commands to: {instance_id}");

            let waiting_result = self.client.wait_until_command_executed()
                .instance_id(instance_id)
                .command_id(command_id)
                .wait(Duration::from_secs(60))
                .await;

            let output = self.client.get_command_invocation()
                .instance_id(instance_id)
                .command_id(command_id)
                .send()
                .await?;

            let stdout = output.standard_output_content().unwrap_or_default();
            info!("STDOUT:\n{stdout}");

            let stderr = output.standard_error_content().unwrap_or_default();
            warn!("STDERR:\n{stderr}");

            let status = output.status().ok_or_else(|| anyhow!("missing status in response"))?;
            info!("command status: {status}");

            if let Err(e) = waiting_result {
                bail!("error waiting: {e:?}");
            }
        }

        Ok(())
    }
}

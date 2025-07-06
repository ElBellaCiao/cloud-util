use crate::InstanceId;
use anyhow::{Result, anyhow, bail};
use aws_sdk_ssm::Client;
use aws_sdk_ssm::client::Waiters;
use aws_sdk_ssm::operation::send_command::SendCommandOutput;
use std::time::Duration;
use tracing::{info, warn};

pub struct SsmClient {
    client: Client,
}

impl SsmClient {
    const WORKING_DIR: &'static str = "/home/ec2-user";

    pub fn builder() -> SsmClientBuilder {
        SsmClientBuilder::default()
    }

    async fn send_command_to_aws(
        &self,
        instance_ids: &[InstanceId],
        commands: Vec<String>,
    ) -> Result<SendCommandOutput> {
        info!("Preparing commands: {commands:?}");

        let instance_id_strings: Vec<String> = instance_ids
            .iter()
            .map(AsRef::as_ref)
            .map(str::to_string)
            .collect();

        info!("Sending commands to instances: {instance_id_strings:?}");

        let response = self
            .client
            .send_command()
            .set_instance_ids(Some(instance_id_strings))
            .document_name("AWS-RunShellScript")
            .parameters("commands", commands)
            .parameters("workingDirectory", vec![Self::WORKING_DIR.to_string()])
            .send()
            .await?;

        Ok(response)
    }
}

#[async_trait::async_trait]
impl crate::manager::Manager for SsmClient {
    async fn send_and_wait(
        &self,
        instance_ids: &[InstanceId],
        commands: Vec<String>,
    ) -> Result<()> {
        let response = self.send_command_to_aws(instance_ids, commands).await?;

        let command = response
            .command()
            .ok_or_else(|| anyhow!("missing command in response"))?;
        let command_id = command
            .command_id()
            .ok_or_else(|| anyhow!("missing command id in response"))?;

        for instance_id in instance_ids {
            info!("Sending commands to: {instance_id}");

            let waiting_result = self
                .client
                .wait_until_command_executed()
                .instance_id(instance_id.to_string())
                .command_id(command_id)
                .wait(Duration::from_secs(60))
                .await;

            let output = self
                .client
                .get_command_invocation()
                .instance_id(instance_id.to_string())
                .command_id(command_id)
                .send()
                .await?;

            let stdout = output.standard_output_content().unwrap_or_default();
            info!("STDOUT:\n{stdout}");

            let stderr = output.standard_error_content().unwrap_or_default();
            warn!("STDERR:\n{stderr}");

            let status = output
                .status()
                .ok_or_else(|| anyhow!("missing status in response"))?;
            info!("command status: {status}");

            if let Err(e) = waiting_result {
                bail!("error waiting: {e:?}");
            }
        }

        Ok(())
    }

    async fn send(&self, instance_ids: &[InstanceId], commands: Vec<String>) -> Result<()> {
        let _ = self.send_command_to_aws(instance_ids, commands).await;
        Ok(())
    }
}

#[derive(Default)]
pub struct SsmClientBuilder {
    client: Option<Client>,
}

impl SsmClientBuilder {
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn build(self) -> SsmClient {
        let client = match self.client {
            Some(client) => client,
            None => {
                let config = aws_config::load_from_env().await;
                Client::new(&config)
            }
        };

        SsmClient { client }
    }
}

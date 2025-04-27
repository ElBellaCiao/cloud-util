use std::time::Duration;
use aws_config::BehaviorVersion;
use aws_sdk_ssm::client::Waiters;
use anyhow::{anyhow, Result};

pub struct Ssm {
    client: aws_sdk_ssm::Client,
}

impl Ssm {
    const WORKING_DIR: &'static str = "/home/ec2-user";

    pub async fn new(client: Option<aws_sdk_ssm::Client>) -> Self {
        let client = match client {
            Some(client) => client,
            None => {
                let config = aws_config::defaults(BehaviorVersion::latest())
                    .load()
                    .await;
                aws_sdk_ssm::Client::new(&config)
            }
        };

        Self { client }
    }

    pub async fn send(&self, instance_ids: &[String], commands: Vec<String>) -> Result<()> {
        let response = self.client.send_command()
            .set_instance_ids(Some(instance_ids.to_vec()))
            .document_name("AWS-RunShellScript")
            .parameters("commands", commands)
            .parameters("workingDirectory", vec![Self::WORKING_DIR.to_string()])
            .send()
            .await?;

        let command = response.command().ok_or_else(|| anyhow!("missing command in response"))?;
        let command_id = command.command_id().ok_or_else(|| anyhow!("missing command id in response"))?;

        for instance_id in instance_ids {
            println!("Sending command to: {instance_id}");

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
            println!("STDOUT:\n{stdout}");

            let stderr = output.standard_error_content().unwrap_or_default();
            println!("STDERR:\n{stderr}");

            let status = output.status().ok_or_else(|| anyhow!("missing status in response"))?;
            println!("command status: {status}");

            if let Err(e) = waiting_result {
                return Err(anyhow!("error waiting: {e}"));
            }
        }

        Ok(())
    }
}
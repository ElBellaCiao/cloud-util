use anyhow::{Result, anyhow};

pub struct SsmParameterClient {
    client: aws_sdk_ssm::Client,
}

impl SsmParameterClient {
    pub async fn new(client: aws_sdk_ssm::Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl crate::config_store::ConfigStore for SsmParameterClient {
    async fn get_parameter(&self, key: &str) -> Result<String> {
        let response = self.client.get_parameter().name(key).send().await?;

        let result = response
            .parameter
            .ok_or_else(|| anyhow!("no value found for {}", key))?
            .value
            .ok_or_else(|| anyhow!("no value found for {}", key))?;

        Ok(result)
    }
}

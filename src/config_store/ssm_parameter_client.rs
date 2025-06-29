use anyhow::{Result, anyhow};
use tokio::runtime::Runtime;

pub struct SyncSsmParameterClient {
    client: aws_sdk_ssm::Client,
    runtime: Runtime,
}

impl SyncSsmParameterClient {
    pub fn new() -> Result<Self> {
        let runtime = Runtime::new()?;
        let config = runtime.block_on(async { aws_config::load_from_env().await });
        let client = aws_sdk_ssm::Client::new(&config);
        Ok(Self { client, runtime })
    }

    pub fn get_parameter(&self, key: &str) -> Result<String> {
        let response = self
            .runtime
            .block_on(async { self.client.get_parameter().name(key).send().await })?;

        let result = response
            .parameter
            .ok_or_else(|| anyhow!("no value found for {}", key))?
            .value
            .ok_or_else(|| anyhow!("no value found for {}", key))?;

        Ok(result)
    }
}

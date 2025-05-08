use anyhow::{anyhow, Result};
use crate::helper::aws_client_or_default;

pub struct SecretsManager {
    client: aws_sdk_secretsmanager::Client,
}

impl SecretsManager {
    pub async fn new(client: Option<aws_sdk_secretsmanager::Client>) -> Self {
        let client = aws_client_or_default(client, aws_sdk_secretsmanager::Client::new).await;
        Self { client }
    }

    pub async fn get_secret(&self, secret_name: &str) -> Result<String> {
        println!("searching for secret {secret_name}");

        let response = self.client.get_secret_value()
            .secret_id(secret_name)
            .send()
            .await?;

        response.secret_string.ok_or_else(|| anyhow!("secret {} not found", secret_name))
    }
}
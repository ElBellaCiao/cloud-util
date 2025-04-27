use aws_sdk_sts::Client;
use anyhow::Result;
use crate::helper::aws_client_or_default;

pub struct StsClient {
    client: Client,
}

impl StsClient {
    pub async fn new(client: Option<Client>) -> Self {
        let client = aws_client_or_default(client, Client::new).await;
        Self { client }
    }

    pub async fn assume_role(&self, role_arn: &str, session_name: &str) -> Result<()> {
        self.client.assume_role()
            .role_arn(role_arn)
            .role_session_name(session_name)
            .send().await?;
        Ok(())
    }
}
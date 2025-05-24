use std::fmt::Display;
use anyhow::Result;

pub struct Ec2Metadata {
    client: reqwest::Client,
    token: String,
}

impl Ec2Metadata {
    pub async fn new(client: Option<reqwest::Client>, token: Option<String>) -> Result<Self> {
        let client = client.unwrap_or_default();

        let token = match token {
            Some(t) => t,
            None => {
                client
                    .put("http://169.254.169.254/latest/api/token")
                    .header("X-aws-ec2-metadata-token-ttl-seconds", "21600")
                    .send()
                    .await?
                    .text()
                    .await?
            }
        };

        Ok(Self { client, token })
    }

    pub async fn request(&self, request: impl Display) -> Result<String> {
        let response = self.client
            .get(format!("http://169.254.169.254/latest/meta-data/{}", &request))
            .header("X-aws-ec2-metadata-token", &self.token)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}

#[async_trait::async_trait]
impl crate::metadata::Metadata for Ec2Metadata {
    async fn get_self_id(&self) -> Result<String> {
        self.request("instance-id").await
    }
}

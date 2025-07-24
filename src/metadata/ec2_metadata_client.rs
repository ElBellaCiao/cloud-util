use anyhow::Result;
use std::fmt::Display;

pub struct Ec2MetadataClient {
    client: reqwest::Client,
    token: String,
}

impl Ec2MetadataClient {
    pub fn builder() -> Ec2MetadataBuilder {
        Ec2MetadataBuilder::default()
    }

    pub async fn request(&self, request: impl Display) -> Result<String> {
        let response = self
            .client
            .get(format!(
                "http://169.254.169.254/latest/meta-data/{}",
                &request
            ))
            .header("X-aws-ec2-metadata-token", &self.token)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}

#[async_trait::async_trait]
impl crate::metadata::Metadata for Ec2MetadataClient {
    async fn get_self_id(&self) -> Result<String> {
        self.request("instance-id").await
    }

    async fn get_tag_value(&self, tag_key: &str) -> Result<String> {
        self.request(format!("tags/instance/{}", tag_key)).await
    }

    async fn get_private_ip(&self) -> Result<String> {
        self.request("local-ipv4").await
    }
}

#[derive(Default)]
pub struct Ec2MetadataBuilder {
    client: Option<reqwest::Client>,
    token: Option<String>,
}

impl Ec2MetadataBuilder {
    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub async fn build(self) -> Result<Ec2MetadataClient> {
        let client = self.client.unwrap_or_default();

        let token = match self.token {
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

        Ok(Ec2MetadataClient { client, token })
    }
}

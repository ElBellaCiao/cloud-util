use anyhow::{Result, bail};
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct RestApi {
    client: Client,
    base_url: String,
}

impl RestApi {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }
}

#[async_trait::async_trait]
impl crate::api::Api for RestApi {
    async fn send_request<T, B>(
        &self,
        method: reqwest::Method,
        url_suffix: &str,
        body: Option<B>,
    ) -> Result<T>
    where
        T: DeserializeOwned + 'static,
        B: Serialize + Send + 'static,
    {
        let url = format!("{}/{}", self.base_url, url_suffix);

        let mut request_builder = self.client.request(method, &url);

        if let Some(body_data) = body {
            request_builder = request_builder.json(&body_data);
        }

        let response = request_builder.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to get error details".to_string());
            bail!("API returned error status {}: {}", status, error_text);
        }

        let data = response.json::<T>().await?;

        Ok(data)
    }
}

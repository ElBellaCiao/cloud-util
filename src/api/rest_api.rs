use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::CloudError;

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
    async fn send_request<T: DeserializeOwned>(
        &self, method: reqwest::Method,
        url_suffix: &str,
        body: Option<impl Serialize + Send>
    ) -> Result<T, CloudError> {
        let url = format!("{}/{}", self.base_url, url_suffix);

        let mut request_builder = self.client.request(method, &url);

        if let Some(body_data) = body {
            request_builder = request_builder.json(&body_data);
        }
        
        let response = request_builder
            .send()
            .await
            .map_err(|e| CloudError::client(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unable to get error details".to_string());
            return Err(CloudError::client(format!("API returned error status {}: {}", status, error_text)));
        }

        let data = response.json::<T>().await
            .map_err(|e| CloudError::client(format!("Failed to parse response: {}", e)))?;

        Ok(data)
    }
}
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::CloudError;

mod rest_api;
pub use rest_api::RestApi;

#[async_trait::async_trait]
pub trait Api: Send + Sync {
    async fn send_request<T: DeserializeOwned>(
        &self, method: reqwest::Method,
        url_suffix: &str,
        body: Option<impl Serialize + Send>
    ) -> Result<T, CloudError>;
}

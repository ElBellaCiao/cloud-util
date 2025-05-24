use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::CloudError;

mod rest_api;
pub use rest_api::RestApi;

#[async_trait::async_trait]
pub trait Api: Send + Sync {
    async fn send_request<T, B>(
        &self, 
        method: reqwest::Method,
        url_suffix: &str,
        body: Option<B>
    ) -> Result<T, CloudError>
    where
        T: DeserializeOwned + 'static,
        B: Serialize + Send + 'static;
}

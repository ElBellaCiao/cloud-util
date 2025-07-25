mod dynamo_db;
pub use dynamo_db::DynamoDbClient;

use crate::model::Keyed;
use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};

#[async_trait::async_trait]
pub trait Table<T>: Send + Sync
where
    T: Serialize + DeserializeOwned + Keyed + Send + Sync,
{
    async fn get_entry(&self, pk: &str, sk: &str) -> Result<T>;
    async fn put_entry(&self, item: T) -> Result<()>;
    async fn get_entries_by_pk(&self, pk: &str) -> Result<Vec<T>>;
}

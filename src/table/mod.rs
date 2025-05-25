mod dynamo_db;
pub use dynamo_db::DynamoDb;

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait Keyed {
    fn pk(&self) -> String;
    fn sk(&self) -> String;
}
#[async_trait::async_trait]
pub trait Table<T>: Send + Sync
where T: Serialize + DeserializeOwned + Keyed + Send + Sync {
    async fn get_entry(&self, pk: &str, sk: &str) -> Result<T>;
    async fn put_entry(&self, item: T) -> Result<()>;
}

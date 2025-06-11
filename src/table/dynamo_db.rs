use crate::Table;
use crate::helper::aws_client_or_default;
use crate::table::Keyed;
use anyhow::{Result, anyhow};
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

pub struct DynamoDb {
    client: Client,
    table_name: String,
}
impl DynamoDb {
    pub async fn new(client: Option<Client>, table_name: String) -> Self {
        let client = aws_client_or_default(client, Client::new).await;
        Self { client, table_name }
    }
}

#[async_trait::async_trait]
impl<T> Table<T> for DynamoDb
where
    T: Serialize + DeserializeOwned + Keyed + Send + Sync + 'static,
{
    async fn get_entry(&self, pk: &str, sk: &str) -> Result<T> {
        let resp = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", AttributeValue::S(pk.to_string()))
            .key("SK", AttributeValue::S(sk.to_string()))
            .send()
            .await?;

        let item = resp
            .item
            .ok_or_else(|| anyhow!("Item not found for {}:{}", pk, sk))?;
        let result = serde_dynamo::from_item(item)?;
        Ok(result)
    }

    async fn put_entry(&self, item: T) -> Result<()> {
        let mut item_map: HashMap<String, AttributeValue> = serde_dynamo::to_item(&item)?;
        item_map.insert("PK".to_string(), AttributeValue::S(item.pk()));
        item_map.insert("SK".to_string(), AttributeValue::S(item.sk()));

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item_map))
            .send()
            .await?;

        Ok(())
    }
}

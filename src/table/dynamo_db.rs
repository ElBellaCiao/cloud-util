use crate::table::Keyed;
use crate::Table;
use anyhow::{anyhow, Result};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_dynamo;
use std::collections::HashMap;

pub struct DynamoDb {
    client: Client,
    table_name: String,
}

#[async_trait::async_trait]
impl<T> Table<T> for DynamoDb
where
    T: Serialize + DeserializeOwned + Keyed + Send + Sync + 'static,
{
    async fn get_entry(&self, pk: String, sk: String) -> Result<T> {
        let resp = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", AttributeValue::S(pk))
            .key("SK", AttributeValue::S(sk))
            .send()
            .await?;

        let item = resp.item.ok_or_else(|| anyhow!("Item not found"))?;
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

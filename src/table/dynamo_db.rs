use crate::Table;
use crate::table::Keyed;
use anyhow::{Result, anyhow};
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

pub struct DynamoDbClient {
    client: Client,
    table_name: String,
}
impl DynamoDbClient {
    pub fn builder() -> DynamoDbClientBuilder {
        DynamoDbClientBuilder::default()
    }
}

#[async_trait::async_trait]
impl<T> Table<T> for DynamoDbClient
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

#[derive(Default)]
pub struct DynamoDbClientBuilder {
    client: Option<Client>,
    table_name: Option<String>,
}

impl DynamoDbClientBuilder {
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn table_name(mut self, table_name: &str) -> Self {
        self.table_name = Some(table_name.to_owned());
        self
    }

    pub async fn build(self) -> Result<DynamoDbClient> {
        let client = match self.client {
            Some(client) => client,
            None => {
                let config = aws_config::load_from_env().await;
                Client::new(&config)
            }
        };

        let table_name = self.table_name.ok_or(anyhow!("Missing table name"))?;

        Ok(DynamoDbClient { client, table_name })
    }
}

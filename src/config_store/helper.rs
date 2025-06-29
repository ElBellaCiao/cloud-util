use crate::SyncSsmParameterClient;
use anyhow::Result;
use serde::de::value::StrDeserializer;
use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, Visitor},
};

pub fn get_config<T>() -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let config_store = SyncSsmParameterClient::new()?;
    let result = T::deserialize(SsmParameterDeserializer::new(config_store))?;
    Ok(result)
}

struct SsmParameterDeserializer {
    client: SyncSsmParameterClient,
}

impl SsmParameterDeserializer {
    pub fn new(client: SyncSsmParameterClient) -> Self {
        Self { client }
    }
}

impl<'de> Deserializer<'de> for SsmParameterDeserializer {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::custom("Only structs supported"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(SsmParameterMapAccess {
            fields,
            index: 0,
            client: self.client,
        })
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

fn snake_case_to_kebab_case(s: &str) -> String {
    s.replace('_', "-")
}

struct SsmParameterMapAccess {
    fields: &'static [&'static str],
    index: usize,
    client: SyncSsmParameterClient,
}

impl<'de> MapAccess<'de> for SsmParameterMapAccess {
    type Error = de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.index < self.fields.len() {
            let field_name_str = self.fields[self.index];
            let field_name = seed.deserialize(StrDeserializer::new(field_name_str))?;
            Ok(Some(field_name))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let field_name = self.fields[self.index];
        self.index += 1;

        let kebab_case_key = snake_case_to_kebab_case(field_name);

        let key_str = self
            .client
            .get_parameter(&kebab_case_key)
            .map_err(de::Error::custom)?;

        let key = seed.deserialize(StrDeserializer::new(&key_str))?;
        Ok(key)
    }
}

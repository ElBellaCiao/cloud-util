use anyhow::{bail, Result};
use derive_more::Display;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Eq, PartialEq, Hash, Display)]
pub struct InstanceId {
    #[display(fmt = "{}", id_str)]
    instance_id: String,
}

impl InstanceId {
    pub fn new(id: impl Into<String>) -> Result<Self> {
        let instance_id = id.into();
        if Self::is_valid(&instance_id) {
            Ok(Self { instance_id })
        } else {
            bail!("Invalid EC2 instance ID: {}", instance_id)
        }
    }

    fn is_valid(id: &str) -> bool {
        let re = Regex::new(r"^i-[0-9a-f]{8}([0-9a-f]{9})?$").unwrap();
        re.is_match(id)
    }
}

impl<'de> Deserialize<'de> for InstanceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        InstanceId::new(s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for InstanceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.instance_id.serialize(serializer)
    }
}

impl AsRef<str> for InstanceId {
    fn as_ref(&self) -> &str {
        &self.instance_id
    }
}

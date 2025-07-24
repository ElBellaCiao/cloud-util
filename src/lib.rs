mod helper;

#[cfg(feature = "instance")]
mod instance;
#[cfg(feature = "instance")]
pub use instance::{Ec2Client, Instance, InstanceId, InstanceMetadata, InstanceState};

#[cfg(feature = "manager")]
mod manager;
#[cfg(feature = "manager")]
pub use manager::{Manager, SsmClient};

#[cfg(feature = "pipeline")]
mod pipeline;
#[cfg(feature = "pipeline")]
pub use pipeline::{CodePipelineClient, Pipeline};

#[cfg(feature = "secretsmanager")]
mod secretsmanager;
#[cfg(feature = "secretsmanager")]
pub use secretsmanager::SecretsManager;

#[cfg(feature = "table")]
mod table;
#[cfg(feature = "table")]
pub use table::{DynamoDbClient, Keyed, Table};

#[cfg(feature = "metadata")]
mod metadata;
#[cfg(feature = "metadata")]
pub use metadata::{Ec2MetadataClient, Metadata};

#[cfg(feature = "api")]
mod api;
#[cfg(feature = "api")]
pub use api::{Api, RestApi};

#[cfg(feature = "config-store")]
mod config_store;
#[cfg(feature = "config-store")]
pub use config_store::{SyncSsmParameterClient, get_config};

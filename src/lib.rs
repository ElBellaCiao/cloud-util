mod helper;

#[cfg(feature = "instance")]
mod instance;
#[cfg(feature = "instance")]
pub use instance::{Ec2, Instance, InstanceId, InstanceMetadata, InstanceState};

#[cfg(feature = "manager")]
mod manager;
#[cfg(feature = "manager")]
pub use manager::{Manager, Ssm};

#[cfg(feature = "code-pipeline")]
mod code_pipeline;
#[cfg(feature = "code-pipeline")]
pub use code_pipeline::CodePipeline;

#[cfg(feature = "secretsmanager")]
mod secretsmanager;
#[cfg(feature = "secretsmanager")]
pub use secretsmanager::SecretsManager;

#[cfg(feature = "table")]
mod table;
#[cfg(feature = "table")]
pub use table::{Table, Keyed, DynamoDb};

#[cfg(feature = "metadata")]
mod metadata;
#[cfg(feature = "metadata")]
pub use metadata::{Ec2Metadata, Metadata};

#[cfg(feature = "api")]
mod api;
#[cfg(feature = "api")]
pub use api::{Api, RestApi};

mod common;
mod helper;
pub use common::CloudError;

#[cfg(feature = "instance")]
mod instance;
#[cfg(feature = "instance")]
pub use instance::{Ec2, Instance, InstanceId};

#[cfg(feature = "ssm")]
mod ssm;
#[cfg(feature = "ssm")]
pub use ssm::Ssm;

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

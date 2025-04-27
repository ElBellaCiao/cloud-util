use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PipelineLambdaPayload {
    #[serde(rename = "CodePipeline.job")]
    pub job: Job,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub data: JobData,
}

#[derive(Debug, Deserialize)]
pub struct JobData {
    #[serde(rename = "actionConfiguration")]
    pub action_configuration: ActionConfiguration,

    #[serde(rename = "inputArtifacts")]
    pub input_artifacts: Vec<Artifact>,

    #[serde(rename = "outputArtifacts")]
    pub output_artifacts: Vec<Artifact>,

    #[serde(rename = "artifactCredentials")]
    pub artifact_credentials: ArtifactCredentials,

    #[serde(rename = "encryptionKey")]
    pub encryption_key: EncryptionKey,
}

#[derive(Debug, Deserialize)]
pub struct ActionConfiguration {
    pub configuration: Configuration,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    #[serde(rename = "FunctionName")]
    pub function_name: String,

    #[serde(rename = "UserParameters")]
    pub user_parameters: String,
}

#[derive(Debug, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub revision: Option<String>,
    pub location: ArtifactLocation,
}

#[derive(Debug, Deserialize)]
pub struct ArtifactLocation {
    #[serde(rename = "type")]
    pub location_type: String,

    #[serde(rename = "s3Location")]
    pub s3_location: Option<S3Location>,
}

#[derive(Debug, Deserialize)]
pub struct S3Location {
    #[serde(rename = "bucketName")]
    pub bucket_name: String,

    #[serde(rename = "objectKey")]
    pub object_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ArtifactCredentials {
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,

    #[serde(rename = "sessionToken")]
    pub session_token: String,

    #[serde(rename = "expirationTime")]
    pub expiration_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct EncryptionKey {
    pub id: String,
    #[serde(rename = "type")]
    pub key_type: String,
}

use anyhow::Result;

mod code_pipeline_client;

pub use code_pipeline_client::CodePipelineClient;

#[async_trait::async_trait]
pub trait Pipeline: Send + Sync {
    async fn post_success(&self, job_id: &str) -> Result<()>;
    async fn post_failure(&self, job_id: &str, msg: &str) -> Result<()>;
}

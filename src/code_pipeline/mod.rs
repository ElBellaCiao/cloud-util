use crate::helper::aws_client_or_default;
use anyhow::Result;
use aws_sdk_codepipeline::Client;
use aws_sdk_codepipeline::types::FailureDetails;
use tracing::info;

pub struct CodePipeline {
    client: Client,
}

impl CodePipeline {
    pub async fn new(client: Option<Client>) -> Self {
        let client = aws_client_or_default(client, Client::new).await;
        Self { client }
    }

    pub async fn post_success(&self, job_id: &str) -> Result<()> {
        info!("Job Succeeded");

        self.client
            .put_job_success_result()
            .job_id(job_id)
            .send()
            .await?;

        info!("Job Success Sent");
        Ok(())
    }

    pub async fn post_failure(&self, job_id: &str, msg: &str) -> Result<()> {
        info!("Job Failed: {msg}");

        let failure_details = FailureDetails::builder().message(msg).build()?;

        self.client
            .put_job_failure_result()
            .job_id(job_id)
            .failure_details(failure_details)
            .send()
            .await?;

        info!("Job Failure Sent");
        Ok(())
    }
}

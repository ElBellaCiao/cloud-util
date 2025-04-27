use aws_sdk_codepipeline::Client;
use anyhow::Result;
use aws_sdk_codepipeline::types::FailureDetails;
use crate::helper::aws_client_or_default;

pub struct CodePipeline {
    client: Client,
}

impl CodePipeline {
    pub async fn new(client: Option<Client>) -> Self {
        let client = aws_client_or_default(client, Client::new).await;
        Self { client }
    }

    async fn post_success(&self, job_id: &str) -> Result<()> {
        println!("Job Succeeded");

        self.client.put_job_success_result()
            .job_id(job_id)
            .send().await?;

        Ok(())
    }

    async fn post_failure(&self, job_id: &str, msg: &str) -> Result<()> {
        println!("Job Failed");

        let failure_details = FailureDetails::builder()
            .message(msg)
            .build()?;

        self.client.put_job_failure_result()
            .job_id(job_id)
            .failure_details(failure_details)
            .send()
            .await?;

        Ok(())
    }
}
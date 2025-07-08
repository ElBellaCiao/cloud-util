use anyhow::Result;
use aws_sdk_codepipeline::Client;
use aws_sdk_codepipeline::types::{FailureDetails, FailureType};
use tracing::info;

pub struct CodePipelineClient {
    client: Client,
}

impl CodePipelineClient {
    pub fn builder() -> CodePipelineClientBuilder {
        CodePipelineClientBuilder::default()
    }
}

#[async_trait::async_trait]
impl crate::pipeline::Pipeline for CodePipelineClient {
    async fn post_success(&self, job_id: &str) -> Result<()> {
        info!("Job Succeeded");

        self.client
            .put_job_success_result()
            .job_id(job_id)
            .send()
            .await?;

        info!("Job Success Sent");
        Ok(())
    }

    async fn post_failure(&self, job_id: &str, msg: &str) -> Result<()> {
        info!("Job Failed: {msg}");

        let failure_details = FailureDetails::builder()
            .r#type(FailureType::JobFailed)
            .message(msg)
            .build()?;

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

#[derive(Default)]
pub struct CodePipelineClientBuilder {
    client: Option<Client>,
}

impl CodePipelineClientBuilder {
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn build(self) -> CodePipelineClient {
        let client = match self.client {
            Some(client) => client,
            None => {
                let config = aws_config::load_from_env().await;
                Client::new(&config)
            }
        };

        CodePipelineClient { client }
    }
}

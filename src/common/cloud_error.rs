use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum CloudError {
    #[error("Client error: {0}")]
    Client(String),

    #[error("Server error: {0}")]
    Server(Box<dyn Error + Send + Sync>),
}

impl CloudError {
    pub fn server<E>(e: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        CloudError::Server(Box::new(e))
    }
}
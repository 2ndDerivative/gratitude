use crate::verification::VerificationError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Header '{0}' not found.")]
    HeaderNotFound(String),

    #[error("Failed to deserialize from or serialize to JSON.")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Invalid payload provided: {0}.")]
    InvalidPayload(String),

    #[error("Verification failed.")]
    VerificationFailed(VerificationError),

    #[error("Worker error: {0}.")]
    WorkerError(worker::Error),
}

impl From<worker::Error> for Error {
    fn from(error: worker::Error) -> Self {
        Self::WorkerError(error)
    }
}

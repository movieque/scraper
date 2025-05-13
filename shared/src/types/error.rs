use serde_json::Error as JsonError;
use aws_sdk_sqs::error::BuildError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("tried to create a message with an empty body")]
    EmptyMessage,
    #[error("JSON Error: {0:?}")]
    JsonError(#[from]JsonError),
    #[error("BUILD Error: {0:?}")]
    BuildError(#[from]BuildError)
}
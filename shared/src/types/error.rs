use aws_sdk_sqs::{config::http::HttpResponse, error::SdkError, operation::send_message_batch::SendMessageBatchError, types::BatchResultErrorEntry};
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
    BuildError(#[from]BuildError),
    #[error("SQS Batch Error: {0:?}")]
    SQSBatchError(Vec<BatchResultErrorEntry>),
    #[error("SQS Sdk Error: {0:?}")]
    SQSSdkError(aws_sdk_sqs::error::SdkError<SendMessageBatchError, HttpResponse>),
}

impl From<&[BatchResultErrorEntry]> for Error {
    fn from(error: &[BatchResultErrorEntry]) -> Self {
        Error::SQSBatchError(error.to_vec())
    }
}


impl From<SdkError<SendMessageBatchError, HttpResponse>> for Error {
    fn from(error: SdkError<SendMessageBatchError, HttpResponse>) -> Self {
        Error::SQSSdkError(error)
    }
}
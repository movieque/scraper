use aws_sdk_sqs::types::SendMessageBatchRequestEntry;
use serde::{Serialize, Deserialize};
use super::{Dataset, Error};


pub type Result<T> = std::result::Result<T, Error>;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub dataset: Dataset,
    pub ids: Vec<u32>,
}

impl TryFrom<Message> for SendMessageBatchRequestEntry {
    type Error = Error;

    fn try_from(message: Message) -> Result<Self> {
        let id = message.dataset.to_string() + "-" + &message.ids.first().ok_or(Error::EmptyMessage)?.to_string() + & match message.ids.last(){
            Some(last) => String::from("-") + &last.to_string(),
            None => String::new()
        };
        let message_body = serde_json::to_string(&message)?;
        let message = SendMessageBatchRequestEntry::builder().id(id).message_body(message_body).build()?;
        Ok(message)
    }
}
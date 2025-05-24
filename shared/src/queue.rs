use aws_sdk_sqs::types::SendMessageBatchRequestEntry;
use aws_config::{load_defaults, BehaviorVersion};
use crate::{Message, Result, Dataset};
use futures::future::try_join_all;
use aws_sdk_sqs::Client;


pub async fn process_ids(dataset: Dataset, ids: impl Iterator<Item = u32>, url: &str) -> Result<()> {
    let messages = create_messages(dataset, ids);
    enqueue_messages(messages, url).await
}


pub async fn enqueue_messages(messages: Vec<Message>, url: &str) -> Result<()> {
    let config = load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    let mut messages = messages.into_iter().peekable();
    let mut futures = Vec::new();
    while let Some(_) = messages.peek() {
        // create the futures into chunks of maximum 500 to create a limit on how many requests are sent to the sqs queue at once.
        let mut iter = Vec::new();
        for _ in 0..500 {
            // create a batch of 10 messages for single http request on each batch.
            let mut batch = Vec::<Message>::new();
            for _ in 0..10 {
                if let Some(message) = messages.next() {
                    batch.push(message);
                }
            }
            if !batch.is_empty() {
                iter.push(queue(batch, &client, url));
            }
        }
        if !iter.is_empty() {
            futures.push(iter)
        }
    }
    for iter in futures {
        try_join_all(iter).await?;
    }
    Ok(())
}



async fn queue(batch: Vec<Message>, client: &Client, url: &str) -> Result<()> {
    let mut entries = Vec::<SendMessageBatchRequestEntry>::new();
    for message in batch {
        let message = message.try_into()?;
        entries.push(message);
    }
    let output = client.send_message_batch().queue_url(url).set_entries(Some(entries)).send().await?;
    if !output.failed().is_empty() {
        return Err(output.failed().into());
    }
    Ok(())
}


fn create_messages(dataset: Dataset, ids: impl Iterator<Item = u32>) -> Vec<Message> {
    let mut iter = ids.peekable();
    let mut messages = Vec::<Message>::new();
    let date = None;
    while let Some(_) = iter.peek() {
        let mut ids = Vec::<u32>::new();
        for _ in 0..50 {
            if let Some(id) = iter.next() {
                ids.push(id);
            }
        }
        if !ids.is_empty() {
            let message = Message{dataset, date, ids};
            messages.push(message);
        }
    }
    messages
}
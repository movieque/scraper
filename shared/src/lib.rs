mod types;

pub use types::*;


pub fn create_messages(dataset: Dataset, ids: impl Iterator<Item = u32>) -> Vec<Message> {
    let mut iter = ids.peekable();
    let mut messages = Vec::<Message>::new();
    while let Some(_) = iter.peek() {
        let mut ids = Vec::<u32>::new();
        for _ in 0..50 {
            if let Some(id) = iter.next() {
                ids.push(id);
            }
        }
        if !ids.is_empty() {
            let message = Message{dataset, ids};
            messages.push(message);
        }
    }
    messages
}
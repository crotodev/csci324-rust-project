/// Represents a message for the consumer
#[derive(Debug)]
#[allow(dead_code)]
pub struct Message {
    pub payload: String,
    pub topic: String,
    pub offset: u64,
}

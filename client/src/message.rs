// Represents a message for the consumer
#[derive(Debug)]
pub struct Message {
    pub payload: String,
    pub topic: String,
    pub offset: u64,
}

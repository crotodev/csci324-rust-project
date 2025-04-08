/// Represents a message in the broker
#[derive(Clone, Debug)]
pub struct Message {
    pub payload: String,
    pub offset: usize,
}
/// Represents a topic in the broker
#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub messages: Vec<Message>,
    pub next_offset: usize,
}

impl Topic {
    pub fn new(name: &str) -> Self {
        Topic {
            name: name.to_string(),
            messages: vec![],
            next_offset: 0,
        }
    }

    /// Publish a message to the topic
    pub fn publish(&mut self, payload: String) {
        let msg: Message = Message {
            payload,
            offset: self.next_offset,
        };
        self.messages.push(msg);
        self.next_offset += 1;
    }

    /// Consume messages from the topic starting from the given offset
    pub fn consume(&self, offset: usize) -> Vec<Message> {
        self.messages
            .iter()
            .filter(|msg| msg.offset >= offset)
            .cloned()
            .collect()
    }
}

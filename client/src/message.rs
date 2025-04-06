// Represents a message for the consumer
pub struct Message {
    pub payload: String,
}

impl std::str::FromStr for Message {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("Empty message")
        } else {
            Ok(Message {
                payload: s.to_string(),
            })
        }
    }
}
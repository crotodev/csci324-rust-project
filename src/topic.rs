/// Represents a topic that can contain multiple partitions.
struct Topic {
    name: String,
    partitions: Vec<Partition>,
}

/// Represents a message within a partition.
struct Message {
    key: String,
    value: String,
    timestamp: u64,
}

/// Represents a partition that holds a list of messages.
struct Partition {
    id: u32,
    messages: Vec<Message>,
}
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::message::Message;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

// This struct represents a generic Consumer that can receive messages from a server.
pub struct Consumer {
    stream: TcpStream,
    topics: HashMap<String, u64>,
}

impl Consumer {
    pub fn new(address: &str) -> Result<Self> {
        let stream: TcpStream =
            TcpStream::connect_timeout(&address.parse()?, Duration::from_secs(5))?;
        stream.set_nodelay(true)?;
        Ok(Consumer {
            stream,
            topics: HashMap::new(),
        })
    }

    /// Adds topics to the consumer. If a topic already exists, it will not be added again.
    pub fn add_topics(&mut self, topics: &[&str]) {
        for topic in topics {
            self.topics.insert(topic.to_string(), 0);
        }
    }

    // Parses a message from the server response
    fn parse_message(line: &str) -> Option<Message> {
        // Extract the offset
        let offset_end: usize = line.find(']')?;
        let offset: u64 = line[1..offset_end].parse::<u64>().ok()?;

        // Extract the topic and payload
        let rest: &str = &line[offset_end + 2..];
        let mut parts: std::str::SplitN<'_, char> = rest.splitn(2, ' ');
        let topic: String = parts.next()?.to_string();
        let payload: String = parts.next()?.to_string();

        Some(Message {
            payload,
            topic,
            offset,
        })
    }

    // Polls the server for messages
    pub fn poll(&mut self) -> Result<Vec<Message>> {
        let mut messages: Vec<Message> = Vec::new();

        let topics: Vec<(String, u64)> = self.topics.iter().map(|(t, o)| (t.clone(), *o)).collect();
        for (topic, offset) in topics {
            let command: String = format!("CONSUME {} {}\n", topic, offset);
            self.stream.write_all(command.as_bytes()).map_err(|e| {
                format!("Failed to send consume command for topic {}: {}", topic, e)
            })?;

            // Set a read timeout for the stream
            self.stream
                .set_read_timeout(Some(Duration::from_secs(10)))
                .map_err(|e| format!("Failed to set read timeout: {}", e))?;

            let reader: BufReader<&TcpStream> = BufReader::new(&self.stream);

            for line in reader.lines() {
                let line: String = match line {
                    Ok(line) => line,
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::WouldBlock
                            || e.kind() == std::io::ErrorKind::TimedOut
                        {
                            break; // Break out of the loop on timeout
                        }
                        return Err(
                            format!("Failed to read response for topic {}: {}", topic, e).into(),
                        );
                    }
                };

                if let Some(message) = Self::parse_message(&line) {
                    println!("{:?}", message);
                    // Update the offset for the topic
                    self.topics.insert(topic.clone(), message.offset + 1);
                    messages.push(message);
                } else {
                    eprintln!("Skipping malformed line for topic {}: {}", topic, line);
                }
            }
        }

        Ok(messages)
    }
}

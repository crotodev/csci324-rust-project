use log::{error, info};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::topic::Topic;

// struct Consumer {
//     pub stream: TcpStream,
//     pub topics: Vec<String>,
//     pub offset: usize,
// }

/// This struct represents a broker that manages multiple topics
pub struct Broker {
    pub topics: HashMap<String, Arc<Mutex<Topic>>>,
    // pub consumers: Vec<Arc<Mutex<Consumer>>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            topics: HashMap::new(),
            // consumers: Vec::new(),
        }
    }

    /// Returns an existing topic or creates a new one if it doesn't exist
    pub fn get_or_create_topic(&mut self, topic: &str) -> Arc<Mutex<Topic>> {
        self.topics
            .entry(topic.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(Topic::new(topic))))
            .clone()
    }
}

/// Handles client connections, reading commands and interacting with the broker
fn handle_client(stream: TcpStream, broker: Arc<Mutex<Broker>>) {
    // Get client address
    let client_addr = match stream.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => "unknown".to_string(),
    };

    // Log the client connection
    let reader: BufReader<&TcpStream> = BufReader::new(&stream);
    for line in reader.lines() {
        let line: String = match line {
            Ok(line) => line,
            Err(e) => {
                error!("[{}] Failed to read line from client: {}", client_addr, e);
                break;
            }
        };

        // Split the line into command, topic, and payload
        let mut parts: std::str::SplitN<'_, char> = line.splitn(3, ' ');
        let command: &str = parts.next().unwrap_or("");
        let topic: &str = parts.next().unwrap_or("");
        let payload: &str = parts.next().unwrap_or("");

        info!(
            "[{}] Received command: {} {} {}",
            client_addr, command, topic, payload
        );

        // Match the command and perform the appropriate action
        match command.to_uppercase().as_str() {
            "PUBLISH" => {
                if topic.is_empty() || payload.is_empty() {
                    error!(
                        "[{}] Invalid PUBLISH command: topic or payload missing",
                        client_addr
                    );
                    if let Err(e) = writeln!(&stream, "Error: Invalid PUBLISH command") {
                        error!(
                            "[{}] Failed to send error message to client: {}",
                            client_addr, e
                        );
                    }
                    continue;
                }
                let mut broker: std::sync::MutexGuard<'_, Broker> = broker.lock().unwrap();
                let topic_ref: Arc<Mutex<Topic>> = broker.get_or_create_topic(topic);
                let mut topic: std::sync::MutexGuard<'_, Topic> = topic_ref.lock().unwrap();
                topic.publish(payload.to_string());
                info!(
                    "[{}] Published message to topic '{}': {}",
                    client_addr, topic.name, payload
                );
            }
            "SUBSCRIBE" => {
                let offset: usize = payload.parse().unwrap_or(0); // Default to 0 if parsing fails
                let messages: Vec<crate::topic::Message>;
                let topic_name: String;

                {
                    let broker: std::sync::MutexGuard<'_, Broker> = broker.lock().unwrap();
                    if let Some(topic_ref) = broker.topics.get(topic) {
                        let topic: std::sync::MutexGuard<'_, Topic> = topic_ref.lock().unwrap();
                        messages = topic.consume(offset);
                        topic_name = topic.name.clone();
                    } else {
                        error!(
                            "[{}] Topic '{}' not found for consumption",
                            client_addr, topic
                        );
                        continue;
                    }
                } 

                // Send messages to the consumer
                for msg in messages {
                    if let Err(e) =
                        writeln!(&stream, "[{}] {} {}", msg.offset, topic_name, msg.payload)
                    {
                        error!("[{}] Failed to send message to client: {}", client_addr, e);
                    }
                }
                info!(
                    "[{}] Consumed messages from topic '{}' starting at offset {}",
                    client_addr, topic_name, offset
                );
            }
            _ => {
                if let Err(e) = writeln!(&stream, "Unknown command") {
                    error!(
                        "[{}] Failed to send error message to client: {}",
                        client_addr, e
                    );
                }
                error!("[{}] Unknown command received: {}", client_addr, command);
            }
        }
    }
    info!("[{}] Client disconnected", client_addr);
}

/// Starts the broker and listens for incoming connections
pub fn start_broker(address: &str) {
    let broker: Arc<Mutex<Broker>> = Arc::new(Mutex::new(Broker::new()));
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    println!("Broker listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let broker: Arc<Mutex<Broker>> = Arc::clone(&broker);
                thread::spawn(move || {
                    handle_client(stream, broker);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

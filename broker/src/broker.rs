// This file contains logging functionality added by AI.
// The logging is implemented using the `log` crate.

use log::{error, info};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::topic::Topic;

/// This struct represents a broker that manages multiple topics
pub struct Broker {
    pub topics: HashMap<String, Arc<Mutex<Topic>>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            topics: HashMap::new(),
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
    // Ref: https://stackoverflow.com/questions/63024046/how-to-access-the-peer-ip-address-in-tokio-tungstenite-0-10
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
                let broker: std::sync::MutexGuard<'_, Broker> = broker.lock().unwrap();
                if let Some(topic_ref) = broker.topics.get(topic) {
                    let topic: std::sync::MutexGuard<'_, Topic> = topic_ref.lock().unwrap();
                    let messages: Vec<crate::topic::Message> = topic.consume(offset);
                    for msg in messages {
                        if let Err(e) =
                            writeln!(&stream, "[{}] {} {}", msg.offset, topic.name, msg.payload)
                        {
                            error!("[{}] Failed to send message to client: {}", client_addr, e);
                        }
                    }
                    info!(
                        "[{}] Consumed messages from topic '{}' starting at offset {}",
                        client_addr, topic.name, offset
                    );
                } else {
                    error!(
                        "[{}] Topic '{}' not found",
                        client_addr, topic
                    );
                }
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

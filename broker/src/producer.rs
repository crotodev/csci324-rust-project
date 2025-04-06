use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::broker::Broker;
use crate::topic::Message;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub struct Producer {
    broker: Arc<Mutex<Broker>>,
}

impl Producer {
    pub fn new(broker: Arc<Mutex<Broker>>) -> Self {
        Producer { broker }
    }

    pub fn send(&self, topic: &str, message: &str) -> Result<()> {
        // Avoid nested locks by minimizing the scope of the lock
        let topic_ref = {
            let mut broker = self.broker.lock().map_err(|e| format!("Failed to lock broker: {}", e))?;
            broker.get_or_create_topic(topic)
        };
        
        // Now lock the topic and publish
        let mut topic = topic_ref.lock().map_err(|e| format!("Failed to lock topic: {}", e))?;
        topic.publish(message.to_string());
        
        Ok(())
    }
    
    // Add a batching method for efficiency
    pub fn send_batch(&self, topic: &str, messages: Vec<String>) -> Result<()> {
        let topic_ref = {
            let mut broker = self.broker.lock().map_err(|e| format!("Failed to lock broker: {}", e))?;
            broker.get_or_create_topic(topic)
        };
        
        let mut topic = topic_ref.lock().map_err(|e| format!("Failed to lock topic: {}", e))?;
        for message in messages {
            topic.publish(message);
        }
        
        Ok(())
    }
}

pub fn run_producer() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return Err("Usage: producer <address> <topic> <message>".into());
    }

    let address = &args[1];
    let topic = &args[2];
    let message = &args[3];

    // Use a connection timeout
    let stream = TcpStream::connect_timeout(&address.parse()?, Duration::from_secs(5))?;
    
    // Set TCP_NODELAY for better performance when sending small messages
    stream.set_nodelay(true)?;
    
    send_message_to_stream(stream, topic, message)?;
    println!("Message published to topic '{}'", topic);
    
    Ok(())
}

// Extract this functionality for better testability and reuse
fn send_message_to_stream(mut stream: TcpStream, topic: &str, message: &str) -> io::Result<()> {
    let command = format!("PUBLISH {} {}\n", topic, message);
    stream.write_all(command.as_bytes())
}

// Create a connection pool for reusing connections
pub struct ProducerPool {
    address: String,
    connections: Vec<TcpStream>,
    max_connections: usize,
}

impl ProducerPool {
    pub fn new(address: String, max_connections: usize) -> Self {
        ProducerPool {
            address,
            connections: Vec::with_capacity(max_connections),
            max_connections,
        }
    }
    
    pub fn get_connection(&mut self) -> Result<TcpStream> {
        // Try to reuse an existing connection
        if let Some(conn) = self.connections.pop() {
            return Ok(conn);
        }
        
        // Create a new connection
        let stream = TcpStream::connect_timeout(&self.address.parse()?, Duration::from_secs(5))?;
        stream.set_nodelay(true)?;
        Ok(stream)
    }
    
    pub fn release_connection(&mut self, stream: TcpStream) {
        if self.connections.len() < self.max_connections {
            self.connections.push(stream);
        }
        // If we've reached max connections, the stream will be dropped here
    }
    
    pub fn send(&mut self, topic: &str, message: &str) -> Result<()> {
        let stream = self.get_connection()?;
        send_message_to_stream(stream.try_clone()?, topic, message)?;
        self.release_connection(stream);
        Ok(())
    }
}


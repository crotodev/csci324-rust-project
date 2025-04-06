use std::env;
use std::error::Error;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::broker::Broker;
use crate::topic::Message;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub struct Consumer {
    broker: Arc<Mutex<Broker>>,
}

impl Consumer {
    pub fn new(broker: Arc<Mutex<Broker>>) -> Self {
        Consumer { broker }
    }

    pub fn poll(&self, topic: &str) -> Result<Vec<Message>> {
        // Acquire the lock only for the scope needed
        let broker = self.broker.lock().map_err(|e| format!("Failed to lock broker: {}", e))?;
        
        if let Some(topic_ref) = broker.topics.get(topic) {
            // Release broker lock before acquiring topic lock to avoid nested locks
            drop(broker);
            
            let mut topic = topic_ref.lock().map_err(|e| format!("Failed to lock topic: {}", e))?;
            Ok(topic.consume_all())
        } else {
            Ok(vec![])
        }
    }
    
    // Add a method to poll with a limit for better control of resource usage
    pub fn poll_with_limit(&self, topic: &str, limit: usize) -> Result<Vec<Message>> {
        let broker = self.broker.lock().map_err(|e| format!("Failed to lock broker: {}", e))?;
        
        if let Some(topic_ref) = broker.topics.get(topic) {
            // Release broker lock before acquiring topic lock
            drop(broker);
            
            let mut topic = topic_ref.lock().map_err(|e| format!("Failed to lock topic: {}", e))?;
            // Assume there's a consume_n method or implement one
            // If not available, we can slice from consume_all
            let all_messages = topic.consume_all();
            Ok(all_messages.into_iter().take(limit).collect())
        } else {
            Ok(vec![])
        }
    }
}

pub fn run_consumer() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err("Usage: consumer <address> <topic>".into());
    }

    let address = &args[1];
    let topic = &args[2];

    // Create a connection with timeout
    let stream = connect_with_retry(address, 3)?;
    
    // Enable keep-alive for long-lived connections
    stream.set_keepalive(Some(Duration::from_secs(60)))?;
    
    // Subscribe to the topic
    subscribe_to_topic(&stream, topic)?;
    
    // Process incoming messages
    read_messages(stream)?;
    
    Ok(())
}

// Helper function to connect with retries
fn connect_with_retry(address: &str, retries: usize) -> Result<TcpStream> {
    let mut attempts = 0;
    let mut last_error = None;
    
    while attempts < retries {
        match TcpStream::connect_timeout(&address.parse()?, Duration::from_secs(5)) {
            Ok(stream) => {
                println!("Connected to broker at {}", address);
                stream.set_nodelay(true)?;
                return Ok(stream);
            }
            Err(e) => {
                eprintln!("Connection attempt {} failed: {}", attempts + 1, e);
                last_error = Some(e);
                attempts += 1;
                if attempts < retries {
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
    
    Err(last_error.unwrap_or_else(|| "Failed to connect after retries".into()).into())
}

// Helper function to subscribe to a topic
fn subscribe_to_topic(stream: &TcpStream, topic: &str) -> io::Result<()> {
    let mut writable_stream = stream.try_clone()?;
    let command = format!("CONSUME {}\n", topic);
    writable_stream.write_all(command.as_bytes())?;
    println!("Subscribed to topic '{}'", topic);
    Ok(())
}

// Helper function to read messages continuously
fn read_messages(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(stream);
    println!("Waiting for messages...");
    
    for line in reader.lines() {
        match line {
            Ok(msg) => {
                if msg.trim().is_empty() {
                    continue;
                }
                println!("Received: {}", msg);
                
                // Here you could add processing logic or dispatch to handlers
            }
            Err(e) => {
                return Err(format!("Error reading from stream: {}", e).into());
            }
        }
    }
    
    Ok(())
}

// Add a more sophisticated consumer with backoff strategy for reconnection
pub struct ResilientConsumer {
    address: String,
    topic: String,
    max_retries: usize,
    handler: Box<dyn Fn(&str) + Send>,
}

impl ResilientConsumer {
    pub fn new<F>(address: String, topic: String, max_retries: usize, handler: F) -> Self 
    where 
        F: Fn(&str) + Send + 'static 
    {
        ResilientConsumer {
            address,
            topic,
            max_retries,
            handler: Box::new(handler),
        }
    }
    
    pub fn start(&self) -> Result<()> {
        let mut consecutive_failures = 0;
        
        loop {
            match self.run_consumer_loop() {
                Ok(_) => {
                    // Reset failure counter on successful connection
                    consecutive_failures = 0;
                    continue;
                }
                Err(e) => {
                    consecutive_failures += 1;
                    
                    if consecutive_failures > self.max_retries {
                        return Err(format!("Exceeded maximum retries ({}): {}", self.max_retries, e).into());
                    }
                    
                    // Exponential backoff
                    let backoff = Duration::from_secs(2u64.pow(consecutive_failures as u32));
                    eprintln!("Connection failed ({}), retrying in {:?}: {}", consecutive_failures, backoff, e);
                    thread::sleep(backoff);
                }
            }
        }
    }
    
    fn run_consumer_loop(&self) -> Result<()> {
        let stream = connect_with_retry(&self.address, 1)?;
        stream.set_keepalive(Some(Duration::from_secs(60)))?;
        
        subscribe_to_topic(&stream, &self.topic)?;
        
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    if !msg.trim().is_empty() {
                        // Call the handler function with the message
                        (self.handler)(&msg);
                    }
                }
                Err(e) => {
                    return Err(format!("Error reading from stream: {}", e).into());
                }
            }
        }
        
        Ok(())
    }
}

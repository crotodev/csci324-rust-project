use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

// This struct represents a generic Producer that can send messages to a server.
pub struct Producer {
    stream: TcpStream,
}

impl Producer {
    pub fn new(address: &str) -> Result<Self> {
        let stream: TcpStream =
            TcpStream::connect_timeout(&address.parse()?, Duration::from_secs(5))?;
        stream.set_nodelay(true)?;
        Ok(Producer { stream })
    }

    /// Sends a message to the specified topic
    pub fn send(&mut self, topic: &str, message: &str) -> Result<()> {
        let command: String = format!("PUBLISH {} {}\n", topic, message);
        self.stream.write_all(command.as_bytes())?;
        self.stream.flush()?;
        Ok(())
    }
}

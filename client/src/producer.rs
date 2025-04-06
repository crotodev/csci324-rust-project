use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub struct Producer {
    stream: TcpStream,
}

impl Producer {
    pub fn new(address: &str) -> Result<Self> {
        let stream = TcpStream::connect_timeout(&address.parse()?, Duration::from_secs(5))?;
        stream.set_nodelay(true)?;
        Ok(Producer { stream })
    }

    pub fn send(&mut self, topic: &str, message: &str) -> Result<()> {
        let command = format!("PUBLISH {} {}\n", topic, message);
        self.stream.write_all(command.as_bytes())?;
        self.stream.flush()?;
        Ok(())
    }
}


CSCI 324 Rust Term Project

This project is a Rust-based message broker system implementing a basic publish-subscribe model over TCP. 
It consists of two main components: a broker (server) and a client that can act as either a producer or consumer. 
The system handles multiple concurrent client connections using threads and shares state safely via Arc<Mutex<>>.

Project Structure

csci324-rust-project/
├── broker/
│   └── src/
│       ├── main.rs        
│       ├── broker.rs      
│       └── topic.rs       
├── client/
│   └── src/
│       ├── main.rs        
│       ├── producer.rs   
│       ├── consumer.rs    
│       └── message.rs    

-------------------------------------------------------------------------------
Submitted Files
-------------------------------------------------------------------------------

File: broker/src/main.rs
Description: Starts broker application. Initializes the server and logger.
Original or Copied: Original
Modification Data: 15 lines

File: broker/src/broker.rs
Description: Defines the core logic for managing topics and client sessions. Handles incoming connections.
Original or Copied: Original
Unoriginal Data: 16 lines
Original: 131 lines

File: broker/src/topic.rs
Description: Topic struct and methods to manage topic-related messages and subscriptions.
Original or Copied: Original
Modification Data: 42 lines

File: client/src/main.rs
Description: CLI for starting the client in producer or consumer mode. Parses args.
Original or Copied: Original
Modification Data: 65 lines

File: client/src/producer.rs
Description: Handles connection to the broker and publishes messages to a topic.
Original or Copied: Original
Modification Data: 29 lines

File: client/src/consumer.rs
Description: Connects to the broker and subscribes to a topic, printing received messages.
Original or Copied: Original
Modification Data: 101 lines 

File: client/src/message.rs
Description: Defines the message struct used by both producer and consumer.
Original or Copied: Original
Modification Data: 8 lines

-------------------------------------------------------------------------------

CSCI 324 Rust Snowman

Project Structure

csci324-rust-project/
├── broker/
│   └── src/
│       ├── main.rs  

-------------------------------------------------------------------------------
Submitted Files
-------------------------------------------------------------------------------

File: src/message.rs
Description: Holds the core game logic and game itself.
Original or Copied: Original
Unoriginal: 5 lines
Original: 283 lines
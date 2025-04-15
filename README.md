# CSCI 324 Rust Message Broker System

This project implements a TCP-based message broker system in Rust, featuring both a broker server and a client capable of acting as a producer or consumer. It is part of the CSCI 324 coursework.

## Project Structure

```
csci324-rust-project/
├── broker/
│   └── src/
│       ├── main.rs        # Starts the message broker
│       ├── broker.rs      # Broker logic and client handling
│       └── topic.rs       # Manages topics and their messages
├── client/
│   └── src/
│       ├── main.rs        # CLI to act as producer or consumer
│       ├── producer.rs    # Produces and sends messages to broker
│       ├── consumer.rs    # Connects and listens for topic messages
│       └── message.rs     # Defines message structure
```

### Building and Running

#### 1. Start the Broker

```bash
cd broker
cargo run -- 127.0.0.1:9092
```

If no address is provided, it defaults to `127.0.0.1:9092`.

#### 2. Run a Producer

```bash
cd client
cargo run -- produce 127.0.0.1:9092 topic 10
```

Or use the shortcut:

```bash
cargo run -- produce test [n] # sends `n` test messages (default 100)
```

#### 3. Run a Consumer

```bash
cargo run -- consume 127.0.0.1:9092 topic
```

The consumer connects to the broker and listens for messages on the specified topic.

### Generating Documentation

To get a basic overview of the functions and data structures in each module, you can generate and view the documentation using `cargo doc`. Follow these steps:

#### 1. Generate and View Broker Documentation

```bash
cd broker
cargo doc --open
```

#### 2. Generate and View Client Documentation

```bash
cd client
cargo doc --open
```

## Logging

To enable logging for broker, run:

```bash
RUST_LOG=info cargo run -- ...
```

## Features

- Basic publish-subscribe messaging model
- Concurrent client handling via threads
- Shared state management using `Arc<Mutex<>>`
- Topic-based message routing
- Client CLI to choose role (producer or consumer)
- Logging via `env_logger`

## License

This project is licensed under the MIT License.

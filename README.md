# Rust Message Broker – v0.1.0

A minimal, TCP-based publish/subscribe message broker written in pure Rust.
It ships with a standalone **broker** binary and a **client** CLI that can act as either a producer or consumer.
The goal is to demonstrate a clean, dependency-light design that still covers core broker features—topic management, message offsets, and concurrent client handling—while remaining easy to extend.

---

## Why it’s interesting

| Technique / concept | Where it shows up | Why you might care |
| ------------------- | ----------------- | ------------------ |
| **`std::net::TcpListener` / `TcpStream`** for raw socket I/O | [`broker/src/broker.rs`](broker/src/broker.rs) | No external networking libs—illustrates zero-cost abstractions and manual back-pressure handling. |
| **`thread::spawn` + `Arc<Mutex<T>>` for shared state** | [`broker/src/broker.rs`](broker/src/broker.rs) | Classic *interior mutability* pattern; lets multiple workers publish to or read from topics concurrently. |
| **Log instrumentation** with [`log`](https://docs.rs/log) + [`env_logger`](https://docs.rs/env_logger) | All binaries (`main.rs`) | Drop-in tracing that respects `RUST_LOG`. Great template for richer observability later. |
| **Command-line dispatch** via `std::env::args()` | [`client/src/main.rs`](client/src/main.rs) | Simple but extensible—swap in `clap` or `argh` when feature set grows. |
| **Offset-based message replay** | [`broker/src/topic.rs`](broker/src/topic.rs) | Mimics Kafka-style semantics; demonstrates how to track and serve historical messages with minimal code. |

*(MDN links are omitted because the code is server-side Rust; MDN primarily covers browser APIs.)*

---

## Not-so-obvious tech in use

| Library / tool | Purpose |
| -------------- | ------- |
| [`env_logger`](https://docs.rs/env_logger) | Runtime-configurable logging without a config file. |

---

## Project layout

```text
csci324-rust-project/
├── broker/          # Rust crate for the TCP broker
│   └── src/
├── client/          # Rust crate for producer/consumer CLI
│   └── src/
├── docs/            # (Implied) design notes or future protocol specs
├── LICENSE
└── README.md
```

- **broker/** – spins up the server, accepts client sockets, and routes messages by topic.
- **client/** – CLI with `produce` and `consume` sub-commands; ideal jumping-off point for tests or load generators.
- **docs/** – reserve for protocol evolution, API contracts, or benchmarking methodology.


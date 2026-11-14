# Tiny K/V Db Server With Rust

An in-memory TCP key-value database that stores a string associated with a key.

> **Note:** This is a personal learning project built from scratch to understand Rust's concurrency, thread pools, and memory safety. It is not intended for production use.

## Features
- **Thread Pool:** Custom thread pool for concurrent operations.
- **Graceful Shutdown:** Implemented Drop trait to safely shutdown the server.
- **Shared State Database:** Uses `Arc<RwLock<T>>` for sharing database between threads.

## Prerequisites
You need to have `Rust` and `Cargo` installed.

```bash
# Clone the repository
git clone https://github.com/MSerox/tiny-kv.git

# Navigate to the folder
cd tiny-kv

# Build release version
cargo build --release
```

## How to Run

1. Start the Server:
Open a terminal and run the server:

```bash
cargo run --release
```

The server will start listening on 127.0.0.1:5302.

2. Connect as a Client:
Open a new terminal window and use telnet (or nc) to connect to the server:

```bash
telnet 127.0.0.1 5302
```

3. Available Commands:
Once connected, you can use the following commands:

- set <key> <value>: Stores a value. (e.g., set name MSerox)

- get <key>: Retrieves a value. (e.g., get name)

- /q: Disconnects from the server.


## What I Learned

- Thread Pool: The basics of the thread pool.
- TCP Server: listen for incoming network connections on a specific port.
- Safely Share Value: How to share a single variable between multiple threads safely.


Distributed under the [MIT License](LICENSE).

# Step 0: The TCP Echo Server 🦀

This branch contains the foundational step of building a Redis clone in Rust: a basic, single-threaded TCP Echo Server. 

Before we can build an in-memory database, we need a server that can bind to a port, accept incoming network connections, and physically read and write bytes over a socket. This step accomplishes exactly that using Rust's standard library.

## 🎯 Objective
Create a server that listens on port `6379` (the default Redis port) and continuously echoes back any bytes it receives from a client until the client disconnects.

## 🧠 Technical Concepts Covered
* **Socket Programming:** Using `std::net::TcpListener` and `TcpStream` to handle TCP connections.
* **Blocking I/O:** The server operates on a single thread and uses blocking loops. It completely processes one client before it can accept the next.
* **Byte Buffers:** Managing raw byte arrays (`[u8; 512]`) to read data from the network stream and write it back.
* **Test-Driven Development (TDD):** Utilizing `std::thread` and `std::sync::Once` to spin up a background test server, allowing automated integration tests to act as mock network clients.

## 🚀 How to Run

### 1. Start the Server
Run the project using Cargo. This will start the server on `127.0.0.1:6379`.
```bash
cargo run
```

### 2. Connect a Client
Open a second terminal window and use netcat (or telnet) to connect to the server:
```bash
nc 127.0.0.1 6379
```
Type any message and press Enter. The server will immediately echo your exact message back to you. Press Ctrl+C to disconnect.

## 🧪 Testing
This step implements full integration tests to verify both the connection acceptance and the echo logic.

Run the test suite:
```bash
cargo test
```

## ⚠️ Limitations (Why isn't this Redis yet?)
* **Single-Client Bottleneck:** Because it relies on standard blocking I/O without multiplexing (like epoll), a second client trying to connect will hang until the first client disconnects.
* **Protocol Ignorance:** If you connect a real `redis-cli` to this server, it will echo the raw Redis Serialization Protocol (RESP) bytes straight back instead of parsing them into actionable database commands.

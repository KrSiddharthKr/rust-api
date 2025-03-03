Blazingly Fast Rust API 🚀

This project is a high-performance Rust API built using Axum and Tokio, optimized for handling 10K-50K concurrent requests with low latency (~1-5ms per request).

📌 Features

✅ Lightweight and fast HTTP server using Axum✅ Optimized for high concurrency using Tokio's multi-threaded runtime✅ Handles massive traffic loads efficiently✅ Supports load testing with Apache Benchmark (ab) and wrk✅ Production-ready with low response time (~1-5ms)

📂 Project Structure
📦 rust-api
├── src
│   ├── main.rs  # Main entry point for the API
├── Cargo.toml   # Dependencies and configuration
├── README.md    # Project documentation

🛠 Installation

1️⃣ Install Rust & Cargo

Ensure you have Rust installed. If not, install it using:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2️⃣ Clone the Repository
git clone https://github.com/yourusername/rust-api.git
cd rust-api



git clone https://github.com/yourusername/rust-api.git
cd rust-api

3️⃣ Install Dependencies
cargo build

🚀 Running the API

Development Mode
cargo run

Optimized for Performance (Production Mode)
cargo run --release

📊 Load Testing

To test how well the API performs under heavy load, use Apache Benchmark (ab) or wrk.

Test with Apache Benchmark (10K Requests, 500 Concurrent)

ab -n 10000 -c 500 http://localhost:3000/

Test with Wrk (50K Requests, 8 Threads, 5K Concurrent)
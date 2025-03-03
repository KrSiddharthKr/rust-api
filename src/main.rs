use axum::{
    routing::get,
    Router,
};
use hyper::Server;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceExt; // Needed for `.into_make_service()`

#[tokio::main(flavor = "multi_thread", worker_threads = 32)] // Increase worker threads for concurrency
async fn main() {
    // Define the router
    let app = Router::new().route("/", get(root_handler));

    // Set the socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("🚀 Server running on {}", addr);

    // High-performance async TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let app_clone = app.clone();

        // Spawn each request on a separate async task
        tokio::spawn(async move {
            if let Err(err) = axum::serve(stream, app_clone.into_make_service()).await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

// Fast response handler
async fn root_handler() -> &'static str {
    "Blazingly fast API response! 🚀"
}

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower::ServiceExt; // Required for `into_make_service()`

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root_handler));

    // Bind to the address with TcpListener instead of TcpStream
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Blazingly fast API response! 🚀"
}

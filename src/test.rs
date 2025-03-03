use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() {
    let client = Client::new();
    for _ in 0..10 {
        let res = client.get("http://127.0.0.1:3000/encrypt")
                        .send().await.unwrap();
        let body = res.text().await.unwrap();
        println!("{}", body);
    }
}

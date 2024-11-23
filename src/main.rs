use tokio;

#[tokio::main]
async fn main() {
    tokio::signal::ctrl_c().await.unwrap();
}

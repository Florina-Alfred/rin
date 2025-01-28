use futures_util::StreamExt;
use std::time::Instant;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let topic = std::env::args()
        .nth(1)
        .expect("add a topic to subscribe to");
    let server = format!("ws://0.0.0.0:3212/ws/sub/{}", topic);
    let ws_stream = match connect_async(server).await {
        Ok((stream, response)) => {
            tracing::info!("Handshake for client has been completed");
            tracing::info!("Server response was {response:?}");
            stream
        }
        Err(e) => {
            tracing::info!("WebSocket handshake for client failed with {e}!");
            return;
        }
    };

    let (_sender, mut receiver) = ws_stream.split();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(txt) = msg {
            let txt = serde_json::from_str::<serde_json::Value>(&txt)
                .expect("Failed to parse JSON message");
            tracing::info!("Received a text message: {}", txt);
            println!("{}", txt);
        }
    }

    let time_elapsed = start_time.elapsed();
    tracing::info!("Client finished in {:.3?}", time_elapsed);
}

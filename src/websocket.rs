use axum::extract::connect_info::ConnectInfo;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use axum_extra::TypedHeader;
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use axum::extract::ws::CloseFrame;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zenoh::Config;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropagationContext(pub HashMap<String, String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpannedMessage {
    pub message: String,
    pub span: PropagationContext,
}

pub fn unspanned_message(
    message: String,
) -> Result<(String, PropagationContext), serde_json::Error> {
    let serialized: SpannedMessage = serde_json::from_str(&message).unwrap();
    Ok((serialized.message, serialized.span))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .route("/ws/{topic_name}", any(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3212").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(topic_name): Path<String>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected for topic {topic_name}.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, topic_name))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, topic_name: String) {
    let (mut sender, _receiver) = socket.split();
    println!("Websocket context {who} created for topic {topic_name}");

    loop {
        let mut config = Config::default();
        config
            .insert_json5("mode", &serde_json::json!("client").to_string())
            .unwrap();

        let _ = config.insert_json5(
            "connect/endpoints",
            &serde_json::json!(vec!["tcp/0.0.0.0:7447"]).to_string(),
        );

        let session = zenoh::open(config).await.unwrap();
        let subscriber = session
            .declare_subscriber(topic_name.as_str())
            .await
            .unwrap();

        while let Ok(sample) = subscriber.recv_async().await {
            let payload = sample
                .payload()
                .try_to_string()
                .unwrap_or_else(|e| e.to_string().into());

            let value = payload.to_string();
            let (value, _) = unspanned_message(value.clone()).unwrap();
            println!("[ {} ] Received {}", who, value);

            if sender
                .send(Message::Text(format!("{value}").into()))
                .await
                .is_err()
            {
                println!("client {who} abruptly disconnected");
                break;
            }
        }
    }
}

mod node;

use axum::extract::{
    connect_info::ConnectInfo,
    ws::{
        Message::{self, Text},
        WebSocket, WebSocketUpgrade,
    },
    Path, State,
};
use axum::{response::IntoResponse, routing::any, Router};
use axum_extra::TypedHeader;
use futures::stream::SplitSink;
use futures::{sink::SinkExt, stream::StreamExt};
use node::common::unspanned_message;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{Receiver, Sender};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zenoh::Config;

#[derive(Clone)]
struct AppState {
    conn_topic_channels: Arc<Mutex<HashMap<String, Vec<Sender<String>>>>>,
    // conn_topic_channels: Arc<Mutex<HashMap<String, Vec<String>>>>,
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

    // let mut state = AppState {
    //     conn_topic_channels: Arc::new(Mutex::new(HashMap::new())),
    // };
    let state = Arc::new(Mutex::new(HashMap::new()));

    let state_clone = Arc::clone(&state);
    let app = Router::new()
        .route("/ws/{topic_name}", any(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(Arc::clone(&state_clone));

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
    State(state): State<Arc<Mutex<HashMap<String, Vec<Sender<String>>>>>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::info!("`{user_agent}` at {addr} connected for topic {topic_name}");

    let mut state_guard = state.lock().unwrap();
    let (tx, rx) = tokio::sync::broadcast::channel::<String>(10);
    if !state_guard.contains_key(&topic_name) {
        tracing::error!("Creating topic {topic_name} in state");
        state_guard.insert(topic_name.clone(), vec![tx.clone()]);
        drop(state_guard);
        let topic_name_clone = topic_name.clone();
        tokio::spawn(async move {
            // subscribe_to_topic(topic_name_clone, tx).await;
            subscribe_to_topic(topic_name_clone, Arc::clone(&state)).await;
        });
    } else {
        state_guard.get_mut(&topic_name).unwrap().push(tx.clone());
        drop(state_guard);
        // let topic_name_clone = topic_name.clone();
        // tokio::spawn(async move {
        //     subscribe_to_topic(topic_name_clone, tx).await;
        // });
    }

    ws.on_upgrade(move |socket| handle_socket(socket, addr, topic_name, rx))
}

// async fn subscribe_to_topic(topic_name: String, tx: Sender<String>) {
async fn subscribe_to_topic(
    topic_name: String,
    state: Arc<Mutex<HashMap<String, Vec<Sender<String>>>>>,
) {
    tracing::error!("Launching a new task to subscribe to topic {topic_name}");
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

        tracing::info!("------------{}:- {} ", topic_name, value);
        // tx.send(value).unwrap();
        let state_guard = state.lock().unwrap();
        if let Some(channels) = state_guard.get(&topic_name) {
            for channel in channels {
                channel.send(value.clone()).unwrap();
            }
        }
        drop(state_guard);
    }
}

async fn handle_socket(
    socket: WebSocket,
    who: SocketAddr,
    topic_name: String,
    mut rx: Receiver<String>,
) {
    let (mut sender, _receiver) = socket.split();

    tracing::info!("Websocket context {who} created for topic {topic_name}");

    while let Ok(value) = rx.recv().await {
        tracing::info!("Sending message to {who} for topic {topic_name}");
        match sender.send(Text(value.into())).await {
            Ok(_) => {}
            Err(e) => {
                tracing::warn!("Receiver socket dropped for topic {topic_name}: {e}");
                break;
            }
        }
    }
}

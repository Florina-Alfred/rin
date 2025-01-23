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
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use zenoh::Config;

#[derive(Clone)]
struct AppState {
    conn_topic_addrs: Arc<Mutex<HashMap<String, Vec<SocketAddr>>>>,
    conn_topic_sender: Arc<Mutex<HashMap<String, Vec<SplitSink<WebSocket, Message>>>>>,
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

    let state = AppState {
        conn_topic_addrs: Arc::new(Mutex::new(HashMap::new())),
        conn_topic_sender: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/ws/{topic_name}", any(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(state);

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
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::info!("`{user_agent}` at {addr} connected for topic {topic_name}");

    let mut conn_topic_addrs_guard = state.conn_topic_addrs.lock().unwrap();
    conn_topic_addrs_guard
        .entry(topic_name.clone())
        .or_insert_with(Vec::new)
        .push(addr);
    tracing::info!("Added {addr} to topic {topic_name} connection list.");
    tracing::warn!("Current connections: {:?}", conn_topic_addrs_guard);
    drop(conn_topic_addrs_guard);

    ws.on_upgrade(move |socket| handle_socket(socket, addr, topic_name, state.clone()))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, topic_name: String, state: AppState) {
    let (mut sender, _receiver) = socket.split();
    // state
    //     .conn_topic_sender
    //     .lock()
    //     .unwrap()
    //     .entry(topic_name.clone())
    //     .or_insert_with(Vec::new)
    //     .push(sender);

    tracing::info!("Websocket context {who} created for topic {topic_name}");

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

        tracing::info!("[ {} ] {}:- {} ", who, topic_name, value);

        // for sender in state
        //     .conn_topic_sender
        //     .lock()
        //     .unwrap()
        //     .get_mut(topic_name.as_str())
        //     .unwrap()
        // {
        //     if sender.send(Text(format!("{value}").into())).is_err() {
        //         let mut conn_topic_addrs_guard = state.conn_topic_addrs.lock().unwrap();
        //         conn_topic_addrs_guard
        //             .get_mut(topic_name.as_str())
        //             .unwrap()
        //             .retain(|x| *x != who);
        //         if conn_topic_addrs_guard
        //             .get(topic_name.as_str())
        //             .unwrap()
        //             .is_empty()
        //         {
        //             conn_topic_addrs_guard.remove(topic_name.as_str());
        //         }
        //         tracing::warn!("Removed {who} from topic {topic_name} connection list.");
        //         tracing::warn!("Current connections: {:?}", conn_topic_addrs_guard);
        //         break;
        //     }
        // }

        if sender.send(Text(format!("{value}").into())).await.is_err() {
            let mut conn_topic_addrs_guard = state.conn_topic_addrs.lock().unwrap();
            conn_topic_addrs_guard
                .get_mut(topic_name.as_str())
                .unwrap()
                .retain(|x| *x != who);
            if conn_topic_addrs_guard
                .get(topic_name.as_str())
                .unwrap()
                .is_empty()
            {
                conn_topic_addrs_guard.remove(topic_name.as_str());
            }
            tracing::warn!("Removed {who} from topic {topic_name} connection list.");
            tracing::warn!("Current connections: {:?}", conn_topic_addrs_guard);
            break;
        }
    }
}

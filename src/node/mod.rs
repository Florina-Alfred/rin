pub mod common;

use common::Message;
use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;
use zenoh::bytes::Encoding;
use zenoh::Config;

#[allow(dead_code)]
pub async fn publish(
    key_expr: &str,
    mut stream: impl Message + Debug + Serialize,
    _attachment: Option<String>,
    mode: &str,
    endpoints: Vec<&str>,
) {
    zenoh::init_log_from_env_or("error");

    let _attachment: Option<String> = None;
    let mut config = Config::default();
    config
        .insert_json5("mode", &json!(mode).to_string())
        .unwrap();
    let _ = config.insert_json5("connect/endpoints", &json!(endpoints).to_string());

    common::logger("Opening session...".to_string());
    let session = zenoh::open(config).await.unwrap();

    common::logger(format!("Declaring Publisher on '{}'...", &key_expr).to_string());
    let publisher = session.declare_publisher(key_expr).await.unwrap();

    common::logger(format!("Sending data: {:?}", stream).to_string());
    let buf = stream.ser();
    common::logger(format!(
        "<< [Publisher] Serialized data ('{}': '{:?}')...",
        &key_expr, buf
    ));
    publisher
        .put(buf)
        .encoding(Encoding::TEXT_PLAIN)
        .attachment(_attachment.clone())
        .await
        .unwrap();
}

#[derive(Debug)]
pub enum CallbackInput {
    TypeString(String),
    TypeInt(i32),
    TypeBytes(u8),
}

#[allow(dead_code)]
pub async fn subscribe<T>(key_expr: &str, mode: &str, endpoints: Vec<&str>, callback: impl Fn(T))
where
    T: Default + Message + Clone + Debug + Serialize + for<'de> serde::Deserialize<'de>,
{
    zenoh::init_log_from_env_or("error");

    let mut config = Config::default();
    config
        .insert_json5("mode", &json!(mode).to_string())
        .unwrap();
    let _ = config.insert_json5("connect/endpoints", &json!(endpoints).to_string());

    common::logger("Opening session...".to_string());
    let session = zenoh::open(config).await.unwrap();

    common::logger(format!("Declaring Subscriber on '{}'...", &key_expr).to_string());
    let subscriber = session.declare_subscriber(key_expr).await.unwrap();

    let msg = T::default();
    while let Ok(sample) = subscriber.recv_async().await {
        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| e.to_string().into());

        common::logger(
            format!(
                ">> [Subscriber] Received {} ('{}': '{}')\n",
                sample.kind(),
                sample.key_expr().as_str(),
                payload
            )
            .to_string(),
        );

        let value = payload.clone().to_string();
        common::logger(format!("Value received: {}", value).to_string());
        msg.deser(value);
        callback(msg.clone());

        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            common::logger(format!(" ({})", att).to_string());
        }
    }
}

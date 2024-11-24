pub mod common;

use serde_json::json;
use zenoh::bytes::Encoding;
use zenoh::Config;

#[allow(dead_code)]
pub async fn publish(
    key_expr: &str,
    stream: impl Iterator<Item = u32>,
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

    for (idx, payload) in stream.enumerate() {
        tokio::time::sleep(tokio::time::Duration::from_nanos(1)).await;
        let buf = format!("[{idx:5}] Value-{payload}");
        common::logger(format!("<< [Publisher] Data ('{}': '{}')...", &key_expr, buf).to_string());
        publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .attachment(_attachment.clone())
            .await
            .unwrap();
    }
    common::logger("Closing publisher...".to_string());
}

#[allow(dead_code)]
// pub async fn subscribe(key_expr: &str, mode: &str, endpoints: Vec<&str>, callback: fn(String)) {
pub async fn subscribe<F>(key_expr: &str, mode: &str, endpoints: Vec<&str>, callback: F)
where
    F: Fn(&dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>>,
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

        let msg = payload.clone().to_string();
        // let msg = Box::new(msg);
        if let Err(e) = callback(&msg) {
            common::logger(format!("Error: {:?}", e).to_string());
        }

        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            common::logger(format!(" ({})", att).to_string());
        }
    }
}

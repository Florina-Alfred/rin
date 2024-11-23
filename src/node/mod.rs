pub mod common;

use serde_json::json;
use zenoh::bytes::Encoding;
use zenoh::Config;

pub async fn publish(
    key_expr: Option<&str>,
    stream: Option<impl Iterator<Item = i32>>,
    attachment: Option<String>,
    mode: Option<&str>,
    endpoints: Option<Vec<&str>>,
) {
    zenoh::init_log_from_env_or("error");

    let key_expr = key_expr.unwrap_or("demo/example/zenoh-rs-pub");
    // let attachment = attachment.unwrap_or("".to_string());
    let attachment: Option<String> = None;
    let mode = mode.unwrap_or("client");
    let endpoints = endpoints.unwrap_or(vec!["tcp/0.0.0.0:7447"]);
    let stream = stream.unwrap();

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
        // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let buf = format!("[{idx:4}] Value-{payload}");
        common::logger(format!("<< [Publisher] Data ('{}': '{}')...", &key_expr, buf).to_string());
        publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .attachment(attachment.clone())
            .await
            .unwrap();
    }
    common::logger("Closing publisher...".to_string());
}

pub async fn subscribe(
    key_expr: Option<&str>,
    mode: Option<&str>,
    endpoints: Option<Vec<&str>>,
    callback: Option<fn(String)>,
) {
    zenoh::init_log_from_env_or("error");

    let key_expr = key_expr.unwrap_or("demo/example/zenoh-rs-pub");
    let mode = mode.unwrap_or("client");
    let endpoints = endpoints.unwrap_or(vec!["tcp/0.0.0.0:7447"]);

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
        if let Some(callback) = callback {
            tokio::spawn(async move {
                callback(msg);
            });
        }

        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            common::logger(format!(" ({})", att).to_string());
        }
    }
}

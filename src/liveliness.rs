use serde_json::json;
use zenoh::sample::SampleKind;
use zenoh::Config;

#[tokio::main]
async fn main() {
    // initiate logging

    let mut config = Config::default();
    config
        .insert_json5("mode", &json!("peer").to_string())
        .unwrap();
    let _ = config.insert_json5(
        "connect/endpoints",
        &json!(["tcp/0.0.0.0:7447"]).to_string(),
    );

    println!("Opening session...");
    let session = zenoh::open(config).await.unwrap();

    let key_expr = "**";
    let history = true;

    println!("Declaring Liveliness Subscriber on '{}'...", &key_expr);

    let subscriber = session
        .liveliness()
        .declare_subscriber(key_expr)
        .history(history)
        .await
        .unwrap();

    println!("Press CTRL-C to quit...");
    while let Ok(sample) = subscriber.recv_async().await {
        match sample.kind() {
            SampleKind::Put => println!(
                ">> [LivelinessSubscriber] New alive token ('{}')",
                sample.key_expr().as_str()
            ),
            SampleKind::Delete => println!(
                ">> [LivelinessSubscriber] Dropped token ('{}')",
                sample.key_expr().as_str()
            ),
        }
    }
}


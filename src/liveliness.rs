use serde_json::json;
use zenoh::sample::SampleKind;
use zenoh::{key_expr::KeyExpr, Config};

#[tokio::main]
async fn main() {
    // Initiate logging
    zenoh::init_log_from_env_or("error");

    // let key_expr: KeyExpr<'static> = "/demo/example/liveliness".parse().unwrap();
    let key_expr: KeyExpr<'static> = "test_topic".parse().unwrap();
    let timeout = std::time::Duration::from_secs(5);
    let history = true;
    let mut config = Config::default();
    let _ = config.insert_json5("mode", &json!("client").to_string());
    let _ = config.insert_json5(
        "connect/endpoints",
        &json!(vec!["tcp/0.0.0.0:7447"]).to_string(),
    );

    println!("Opening session...");
    let session = zenoh::open(config).await.unwrap();

    println!("Sending Liveliness Query '{key_expr}'...");
    let subscriber = session
        .liveliness()
        .declare_subscriber(&key_expr)
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


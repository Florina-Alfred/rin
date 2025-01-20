use serde_json::json;
use zenoh::session::ZenohId;
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
    let session = zenoh::open(config).await.unwrap();

    let info = session.info();
    println!("zid: {}", info.zid().await);
    println!(
        "routers zid: {:?}",
        info.routers_zid().await.collect::<Vec<ZenohId>>()
    );
    println!(
        "peers zid: {:?}",
        info.peers_zid().await.collect::<Vec<ZenohId>>()
    );

    let subscriber = session.declare_subscriber("**").await.unwrap();

    println!("Press CTRL-C to quit...");
    while let Ok(sample) = subscriber.recv_async().await {
        // Refer to z_bytes.rs to see how to deserialize different types of message
        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| e.to_string().into());

        print!(
            ">> [Subscriber] Received {} ('{}': '{}')",
            sample.kind(),
            sample.key_expr().as_str(),
            payload
        );
        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            print!(" ({})", att);
        }
        println!();
    }
}



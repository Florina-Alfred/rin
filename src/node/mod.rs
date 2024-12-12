pub mod common;

use common::Message;
use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;
use zenoh::bytes::Encoding;
// use zenoh::handlers::FifoChannelHandler;
use tracing::{debug, info};
use zenoh::Config;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Publisher<'a> {
    session: zenoh::Session,
    attachment: Option<String>,
    publisher: zenoh::pubsub::Publisher<'a>,
}

#[allow(dead_code)]
impl<'a> Publisher<'a> {
    // #[tracing::instrument]
    pub async fn new(
        key_expr: &'a str,
        mode: &'a str,
        attachment: Option<String>,
        endpoints: Vec<&'a str>,
    ) -> Result<Publisher<'a>, zenoh::Error> {
        // zenoh::init_log_from_env_or("error");

        let mut config = Config::default();
        config
            .insert_json5("mode", &json!(mode).to_string())
            .unwrap();
        let _ = config.insert_json5("connect/endpoints", &json!(endpoints).to_string());

        common::logger("Opening session...".to_string());
        let session = zenoh::open(config).await.unwrap();
        let publisher = session.declare_publisher(key_expr).await.unwrap();
        common::logger(format!("Publisher: {:?}", publisher).to_string());
        Ok(Publisher {
            session,
            attachment,
            publisher,
        })
    }
    #[tracing::instrument]
    pub async fn publish(&self, message: impl Message + Debug + Serialize) {
        common::logger(format!("Publishing message: {:?}", message).to_string());
        let buf = message.ser();
        self.publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .attachment(self.attachment.clone())
            .await
            .unwrap();
    }
}

#[allow(dead_code)]
#[tracing::instrument]
pub async fn start_publisher(
    key_expr: &str,
    mut stream: impl Message + Debug + Serialize,
    attachment: Option<String>,
    mode: &str,
    endpoints: Vec<&str>,
) {
    // zenoh::init_log_from_env_or("error");

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

    loop {
        let buf = stream.ser();
        common::logger(format!(
            "<< [Publisher] Serialized data ('{}': '{:?}')...",
            &key_expr, buf
        ));
        publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .attachment(attachment.clone())
            .await
            .unwrap();
        match stream.next().await {
            Some(_) => (),
            None => break,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Subscriber<T> {
    session: zenoh::Session,
    default: T,
    // subscriber: zenoh::pubsub::Subscriber<zenoh::handlers::fifo::FifoChannelHandler<T>>,
    subscriber:
        zenoh::pubsub::Subscriber<zenoh::handlers::fifo::FifoChannelHandler<zenoh::sample::Sample>>,
}

#[allow(dead_code)]
impl<T> Subscriber<T> {
    // #[tracing::instrument]
    pub async fn new(
        key_expr: &str,
        mode: &str,
        _default: T,
        endpoints: Vec<&str>,
    ) -> Result<Subscriber<T>, zenoh::Error>
    where
        T: Default + Message + Clone + Debug + Serialize + for<'de> serde::Deserialize<'de>,
    {
        // zenoh::init_log_from_env_or("error");

        let mut config = Config::default();
        config
            .insert_json5("mode", &json!(mode).to_string())
            .unwrap();
        let _ = config.insert_json5("connect/endpoints", &json!(endpoints).to_string());

        common::logger("Opening session...".to_string());
        let session = zenoh::open(config).await.unwrap();
        let subscriber = session.declare_subscriber(key_expr).await.unwrap();

        Ok(Subscriber::<T> {
            session,
            default: T::default(),
            subscriber,
        })
    }
    #[tracing::instrument]
    pub async fn receive_msg(&self) -> Result<T, zenoh::Error>
    where
        T: Default + Message + Clone + Debug + Serialize + for<'de> serde::Deserialize<'de>,
    {
        let sample = self.subscriber.recv_async().await.unwrap();
        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| e.to_string().into());

        let value = payload.clone().to_string();

        return Ok(self.default.deser(&value));
    }
}

#[allow(dead_code)]
#[tracing::instrument]
pub async fn start_subscriber<T>(
    key_expr: &str,
    mode: &str,
    endpoints: Vec<&str>,
    callback: Vec<fn(T)>,
) where
    T: Default + Message + Clone + Debug + Serialize + for<'de> serde::Deserialize<'de>,
{
    // zenoh::init_log_from_env_or("error");

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
        let msg = T::default();
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
        for f in &callback {
            f(msg.deser(&value));
        }

        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            common::logger(format!(" ({})", att).to_string());
        }
    }
}

#[allow(dead_code)]
#[tracing::instrument]
pub async fn start_subscriber_publisher<T, S>(
    key_expr_sub: &str,
    key_expr_pub: &str,
    mode: &str,
    endpoints: Vec<&str>,
    maniputater: fn(T) -> S,
) where
    T: Default + Message + Clone + Debug + Serialize + for<'de> serde::Deserialize<'de>,
    S: Default + Message + Clone + Debug + Serialize + for<'de> serde::Deserialize<'de>,
{
    // zenoh::init_log_from_env_or("error");

    let mut config = Config::default();
    config
        .insert_json5("mode", &json!(mode).to_string())
        .unwrap();
    let _ = config.insert_json5("connect/endpoints", &json!(endpoints).to_string());

    common::logger("Opening session...".to_string());
    let session = zenoh::open(config).await.unwrap();

    common::logger(format!("Declaring Subscriber on '{}'...", &key_expr_sub).to_string());
    let subscriber = session.declare_subscriber(key_expr_sub).await.unwrap();
    let publisher = session.declare_publisher(key_expr_pub).await.unwrap();

    while let Ok(sample) = subscriber.recv_async().await {
        let msg = T::default();
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
        let manipulated_message = maniputater(msg.deser(&value));

        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            common::logger(format!("({})", att).to_string());
        }

        let buf = manipulated_message.ser();
        common::logger(format!(
            "<< [Publisher] Serialized data ('{}': '{:?}')...",
            &key_expr_pub, buf
        ));
        publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .await
            .unwrap();
    }
}

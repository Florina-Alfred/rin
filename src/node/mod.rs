pub mod common;

use common::Message;
use common::{spanned_message, unspanned_message};
use opentelemetry::trace::{TraceContextExt, Tracer};
use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;
use tracing::{debug, info, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use zenoh::bytes::Encoding;
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
    name: &str,
    key_expr: &str,
    mut payload: impl Message + Debug + Serialize,
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

    common::logger(format!("Opening session for '{}'", &name).to_string());
    let session = zenoh::open(config).await.unwrap();

    common::logger(format!("Declaring {} Publisher on '{}'...", &name, &key_expr).to_string());
    let publisher = session.declare_publisher(key_expr).await.unwrap();

    common::logger(format!("{} ending data: {:?}", &name, &payload).to_string());

    loop {
        // let buf = payload.ser();
        println!(
            "Current span: {:?} with context {:?}",
            tracing::Span::current(),
            tracing::Span::current().context()
        );
        let span = info_span!("Sending data", payload = ?payload);
        let buf = spanned_message(payload.ser(), span);
        common::logger(format!(
            "<< [{:>16}] Serialized data ('{}': '{:?}')...",
            &name, &key_expr, &buf
        ));
        publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .attachment(attachment.clone())
            .await
            .unwrap();

        match payload.next().await {
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
// #[tracing::instrument]
pub async fn start_subscriber<T>(
    name: &str,
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

    common::logger(format!("Opening session for '{}'", &name).to_string());
    let session = zenoh::open(config).await.unwrap();

    common::logger(format!("Declaring {} Subscriber on '{}'...", &name, &key_expr).to_string());
    let subscriber = session.declare_subscriber(key_expr).await.unwrap();

    while let Ok(sample) = subscriber.recv_async().await {
        let msg = T::default();
        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| e.to_string().into());
        // let (payload, span) = common::unspanned_message(payload.to_string()).unwrap();
        // let current_span = tracing::Span::none();
        // current_span.set_parent(span.extract());

        common::logger(
            format!(
                ">> [{:>16}] Received {} ('{}': '{}')\n",
                &name,
                sample.kind(),
                sample.key_expr().as_str(),
                payload
            )
            .to_string(),
        );
        info!("Received data: {}", payload);

        let value = payload.to_string();
        for f in &callback {
            // f(msg.deser(&value));
            let (value, span) = unspanned_message(value.clone()).unwrap();
            let parent_context = span.extract("start_subscriber with callback");
            let span = info_span!("Received data", payload = ?value);
            span.set_parent(parent_context);

            // span.in_scope(|| f(msg.deser(&value)));
            span.in_scope(|| {
                f(msg.deser(&value));
            });

            // f(msg.deser(&value));
            // callback_caller(*f, msg.deser(&value), value);
        }
        // loop_callbacks(msg, payload.to_string(), callback.clone());

        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            common::logger(format!(" ({})", att).to_string());
        }
        // break;
    }
}

#[tracing::instrument]
fn loop_callbacks<T>(msg: T, payload: String, callback: Vec<fn(T)>)
where
    T: Default + Message + Clone + Debug + Serialize + for<'de> serde::Deserialize<'de>,
{
    for f in &callback {
        callback_caller(*f, msg.clone(), payload.clone());
    }
}

#[tracing::instrument]
fn callback_caller<T>(callback: fn(T), msg: T, payload: String)
where
    T: Default + Message + Clone + Debug + Serialize + for<'de> serde::Deserialize<'de>,
{
    let value = payload.to_string();
    callback(msg.deser(&value));
}

#[allow(dead_code)]
#[tracing::instrument]
pub async fn start_subscriber_publisher<T, S>(
    name: &str,
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

    common::logger(format!("Opening session for '{}'", &name).to_string());
    let session = zenoh::open(config).await.unwrap();

    common::logger(
        format!(
            "Declaring {} Subscriber-Publisher on '{}'...",
            &name, &key_expr_sub
        )
        .to_string(),
    );
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
                ">> [{:>16}] Received {} ('{}': '{}')\n",
                &name,
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
            "<< [{:>16}] Serialized data ('{}': '{:?}')...",
            &name, &key_expr_pub, buf
        ));
        publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .await
            .unwrap();
    }
}

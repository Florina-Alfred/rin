pub mod common;

use common::SessionInfo;
use common::{spanned_message, unspanned_message};
use common::{Message, Metric};
// use serde::Serialize;
use serde_json::json;
use std::fmt::Debug;
use tracing::{info, info_span, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use zenoh::bytes::Encoding;
use zenoh::Config;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Publisher<'a> {
    session: zenoh::Session,
    attachment: Option<String>,
    publisher: zenoh::pubsub::Publisher<'a>,
    pub token: zenoh::liveliness::LivelinessToken,
}

#[allow(dead_code)]
impl<'a> Publisher<'a> {
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
        let token = session
            .liveliness()
            .declare_token(format!("{}/pub/{}", key_expr, session.info().zid().await).as_str())
            .await
            .unwrap();
        common::logger(format!("Publisher: {:?}", publisher).to_string());
        Ok(Publisher {
            session,
            attachment,
            publisher,
            token,
        })
    }
    #[tracing::instrument]
    pub async fn publish(&self, message: impl Message + Debug + Metric) {
        common::logger(format!("Publishing message: {:?}", message).to_string());
        info!("Publishing data: {:?}", message);
        // let buf = message.ser();
        let span = info_span!("Sending data", message = ?message);
        info!("Sending data: {:?}", message);
        let buf = spanned_message(message.ser(), span);

        self.publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .attachment(self.attachment.clone())
            .await
            .unwrap();
    }
}

#[allow(dead_code)]
pub async fn start_publisher(
    name: &str,
    key_expr: &str,
    mut payload: impl Message + Debug + Metric,
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

    let mut pub_session_info = SessionInfo {
        zid: format!("{}", session.info().zid().await),
        routers_zid: Vec::new(),
        peers_zid: None,
    };

    for zid in session.info().routers_zid().await {
        pub_session_info.routers_zid.push(format!("{}", zid));
    }

    for zid in session.info().peers_zid().await {
        pub_session_info.peers_zid = Some(vec![format!("{}", zid)]);
    }

    let db = common::DB::new();
    let _ = db.conn.execute(
        "INSERT INTO state (name, topic) VALUES (?1, ?2)",
        (&name.to_string(), &key_expr.to_string()),
    );

    common::logger(format!("Declaring {} Publisher on '{}'...", &name, &key_expr).to_string());
    let publisher = session.declare_publisher(key_expr).await.unwrap();
    let token = session
        .liveliness()
        .declare_token(
            format!("{}/pub/{}/{}", &key_expr, &name, session.info().zid().await).as_str(),
        )
        .await
        .unwrap();

    common::logger(format!("{} ending data: {:?}", &name, &payload).to_string());

    loop {
        let span = info_span!("Sending data", payload = ?payload);
        info!("Sending data: {:?}", payload);
        let buf = spanned_message(payload.ser(), span);
        // common::logger(format!(
        //     "<< [{:>16}] Serialized data ('{}': '{:?}')...",
        //     &name, &key_expr, &buf
        // ));
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
    db.conn
        .execute(
            "DELETE FROM state WHERE name = ?1 AND topic = ?2",
            (&name.to_string(), &key_expr.to_string()),
        )
        .unwrap();
    token.undeclare().await.unwrap();
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Subscriber<T> {
    session: zenoh::Session,
    default: T,
    subscriber:
        zenoh::pubsub::Subscriber<zenoh::handlers::fifo::FifoChannelHandler<zenoh::sample::Sample>>,
    pub token: zenoh::liveliness::LivelinessToken,
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
        T: Default + Message + Metric + Clone + Debug,
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
        let token = session
            .liveliness()
            .declare_token(format!("{}/sub/{}", key_expr, session.info().zid().await).as_str())
            .await
            .unwrap();

        Ok(Subscriber::<T> {
            session,
            default: T::default(),
            subscriber,
            token,
        })
    }
    #[tracing::instrument]
    pub async fn receive_msg(&self) -> Result<T, zenoh::Error>
    where
        T: Default + Message + Metric + Clone + Debug,
    {
        let sample = self.subscriber.recv_async().await.unwrap();
        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| e.to_string().into());

        // let value = payload.clone().to_string();
        // info!("Received data: {}", value);
        // return Ok(self.default.deser(&value));

        let value = payload.to_string();
        // f(msg.deser(&value));
        let (value, span) = unspanned_message(value.clone()).unwrap();
        let parent_context = span.extract();
        let span = info_span!("Received data", payload = ?value);
        span.set_parent(parent_context);

        let msg = T::default();
        let message = msg.deser(&value);
        Ok(message)
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
    T: Default + Message + Metric + Clone + Debug,
{
    // zenoh::init_log_from_env_or("error");

    let mut config = Config::default();
    config
        .insert_json5("mode", &json!(mode).to_string())
        .unwrap();
    let _ = config.insert_json5("connect/endpoints", &json!(endpoints).to_string());

    common::logger(format!("Opening session for '{}'", &name).to_string());
    let session = zenoh::open(config).await.unwrap();

    let db = common::DB::new();
    let _ = db.conn.execute(
        "INSERT INTO state (name, topic) VALUES (?1, ?2)",
        (&name.to_string(), &key_expr.to_string()),
    );

    common::logger(format!("Declaring {} Subscriber on '{}'...", &name, &key_expr).to_string());
    let subscriber = session.declare_subscriber(key_expr).await.unwrap();
    let token = session
        .liveliness()
        .declare_token(
            format!("{}/sub/{}/{}", &key_expr, &name, session.info().zid().await).as_str(),
        )
        .await
        .unwrap();

    while let Ok(sample) = subscriber.recv_async().await {
        let msg = T::default();
        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| e.to_string().into());

        // common::logger(
        //     format!(
        //         ">> [{:>16}] Received {} ('{}': '{}')\n",
        //         &name,
        //         sample.kind(),
        //         sample.key_expr().as_str(),
        //         payload
        //     )
        //     .to_string(),
        // );
        info!("Received data: {}", payload);

        let value = payload.to_string();
        // f(msg.deser(&value));
        let (value, span) = unspanned_message(value.clone()).unwrap();
        let parent_context = span.extract();
        let span = info_span!("Received data", payload = ?value);
        span.set_parent(parent_context);

        let message = msg.deser(&value);
        if message.collect_metrics().is_some() {
            info!("Metric data: {:?}", message.collect_metrics());
        }
        // info!("Metric data: {:?}", message.collect_metrics());
        match message.collect_metrics() {
            Some(metrics) => {
                for (key, value) in metrics {
                    // let _ = span.record(key.as_str(), &value.as_str());
                    let metrics_key = key.clone();
                    let metric_name = "rin";
                    // let metric_name = format!("{}::{}", args::Args::parse().project, "zenoh")
                    //     .as_str()
                    //     .as_ref();
                    let metric = opentelemetry::global::meter(&metric_name);
                    match value.parse::<u64>() {
                        Ok(value) => {
                            let metrics_key_header = metrics_key.clone();
                            let gauge = metric.u64_gauge(metrics_key).build();
                            gauge.record(
                                value as u64,
                                &[opentelemetry::KeyValue::new(
                                    "rin_metric",
                                    format!("{}", metrics_key_header),
                                )],
                            );
                        }
                        Err(_) => {
                            warn!(
                                "Failed to {} metric parse value to u64: {}",
                                metrics_key, value
                            );
                            let metrics_key_header = metrics_key.clone();
                            let gauge = metric.u64_gauge(metrics_key).build();
                            gauge.record(
                                0,
                                &[opentelemetry::KeyValue::new(
                                    "rin_metric",
                                    format!("{}", metrics_key_header),
                                )],
                            );
                        }
                    }
                }
            }
            None => (),
        }

        for f in &callback {
            span.in_scope(|| {
                f(msg.deser(&value));
            });
        }

        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            common::logger(format!(" ({})", att).to_string());
        }
    }
    token.undeclare().await.unwrap();
    db.conn
        .execute(
            "DELETE FROM state WHERE name = ?1 AND topic = ?2",
            (&name.to_string(), &key_expr.to_string()),
        )
        .unwrap();
}

#[allow(dead_code)]
pub async fn start_subscriber_publisher<T, S>(
    name: &str,
    key_expr_sub: &str,
    key_expr_pub: &str,
    mode: &str,
    endpoints: Vec<&str>,
    maniputater: fn(T) -> S,
) where
    T: Default + Message + Metric + Clone + Debug,
    S: Default + Message + Metric + Clone + Debug,
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
    let token_sub = session
        .liveliness()
        .declare_token(
            format!(
                "{}/sub/{}/{}",
                &key_expr_sub,
                &name,
                session.info().zid().await
            )
            .as_str(),
        )
        .await
        .unwrap();
    let publisher = session.declare_publisher(key_expr_pub).await.unwrap();
    let token_pub = session
        .liveliness()
        .declare_token(
            format!(
                "{}/pub/{}/{}",
                &key_expr_pub,
                &name,
                session.info().zid().await
            )
            .as_str(),
        )
        .await
        .unwrap();

    while let Ok(sample) = subscriber.recv_async().await {
        let msg = T::default();
        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| e.to_string().into());

        // common::logger(
        //     format!(
        //         ">> [{:>16}] Received {} ('{}': '{}')\n",
        //         &name,
        //         sample.kind(),
        //         sample.key_expr().as_str(),
        //         payload
        //     )
        //     .to_string(),
        // );

        let value = payload.to_string();
        // f(msg.deser(&value));
        let (value, span) = unspanned_message(value.clone()).unwrap();
        let parent_context = span.extract();
        let span = info_span!("Received data to send again ", payload = ?value);
        info!(
            "Received data to send again: {} at topic {}",
            value, key_expr_sub
        );
        span.set_parent(parent_context);

        let manipulated_message = span.in_scope(|| maniputater(msg.deser(&value)));

        if let Some(att) = sample.attachment() {
            let att = att.try_to_string().unwrap_or_else(|e| e.to_string().into());
            common::logger(format!("({})", att).to_string());
        }

        // common::logger(format!(
        //     "<< [{:>16}] Serialized data ('{}': '{:?}')...",
        //     &name, &key_expr_pub, buf
        // ));
        info!(
            "Sending data again: {:?} at topic {}",
            payload, key_expr_pub
        );
        let buf = spanned_message(manipulated_message.ser(), span);
        publisher
            .put(buf)
            .encoding(Encoding::TEXT_PLAIN)
            .await
            .unwrap();
    }
    token_sub.undeclare().await.unwrap();
    token_pub.undeclare().await.unwrap();
}

use gstreamer::prelude::*;
use gstreamer::MessageView;
use gstreamer::{ElementFactory, Pipeline, State};
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub async fn start_video_pub(device: &str, location: &str) {
    gstreamer::init().expect("Failed to initialize GStreamer");
    let pipeline = Pipeline::with_name("v4l2-rtsp-stream");

    let v4l2src = ElementFactory::make("v4l2src")
        .name("v4l2src")
        .property_from_str("device", device)
        .build()
        .expect("Failed to create v4l2src element");

    let videoconvert = ElementFactory::make("videoconvert")
        .name("videoconvert")
        .build()
        .expect("Failed to create videoconvert element");

    let x264enc = ElementFactory::make("x264enc")
        .name("x264enc")
        .property_from_str("tune", "zerolatency") // Tune for low latency
        .property_from_str("speed-preset", "ultrafast") // Use ultrafast encoding speed
        .build()
        .expect("Failed to create x264enc element");

    let rtspclientsink = ElementFactory::make("rtspclientsink")
        .name("rtspclientsink")
        .property_from_str(
            "location",
            format!("rtsp://0.0.0.0:8554/{}", location).as_str(),
        )
        .property_from_str("protocols", "tcp") // Set to TCP
        .build()
        .expect("Failed to create rtspclientsink element");

    pipeline
        .add_many(&[&v4l2src, &videoconvert, &x264enc, &rtspclientsink])
        .expect("Failed to add elements to the pipeline");

    gstreamer::Element::link_many([&v4l2src, &videoconvert, &x264enc, &rtspclientsink]).unwrap();

    pipeline
        .set_state(State::Playing)
        .expect("Failed to set pipeline to Playing");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        match msg.view() {
            gstreamer::MessageView::Eos(..) => {
                println!("End of Stream");
                break;
            }
            gstreamer::MessageView::Error(err) => {
                eprintln!(
                    "Error: {}: {}",
                    err.error(),
                    err.debug()
                        .unwrap_or_else(|| String::from("No debug info").into())
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(State::Null)
        .expect("Failed to set pipeline to Null");
}

#[allow(dead_code)]
pub async fn start_video_sub(location: &str) {
    gstreamer::init().expect("Failed to initialize GStreamer");

    let pipeline = Pipeline::with_name("test-pipeline");

    let rtspsrc = ElementFactory::make("rtspsrc")
        .name("rtspsrc")
        .property_from_str(
            "location",
            format!("rtsp://0.0.0.0:8554/{}", location).as_str(),
        )
        .property_from_str("latency", "0")
        .build()
        .expect("Failed to create rtspsrc element");

    let decodebin = ElementFactory::make("decodebin")
        .name("decodebin")
        .build()
        .expect("Failed to create decodebin element");

    let videoconvert = ElementFactory::make("videoconvert")
        .name("videoconvert")
        .build()
        .expect("Failed to create videoconvert element");

    let autovideosink = ElementFactory::make("autovideosink")
        .name("autovideosink")
        .build()
        .expect("Failed to create autovideosink element");

    let videoconvert = Arc::new(Mutex::new(videoconvert));

    pipeline
        .add_many(&[
            &rtspsrc,
            &decodebin,
            &videoconvert.lock().unwrap(),
            &autovideosink,
        ])
        .expect("Failed to add elements to the pipeline");

    rtspsrc.connect_pad_added({
        let decodebin = decodebin.clone();
        move |_, pad| {
            let decodebin_pad = decodebin.static_pad("sink").unwrap();
            pad.link(&decodebin_pad)
                .expect("Failed to link rtspsrc to decodebin");
        }
    });

    decodebin.connect_pad_added({
        let videoconvert = Arc::clone(&videoconvert);
        move |_, pad| {
            if let Some(sinkpad) = videoconvert.lock().unwrap().static_pad("sink") {
                pad.link(&sinkpad)
                    .expect("Failed to link decodebin to videoconvert");
            }
        }
    });

    videoconvert
        .lock()
        .unwrap()
        .link(&autovideosink)
        .expect("Failed to link videoconvert to autovideosink");

    pipeline
        .set_state(State::Playing)
        .expect("Failed to set pipeline to Playing");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => {
                println!("End of Stream");
                break;
            }
            MessageView::Error(err) => {
                eprintln!(
                    "Error: {}: {}",
                    err.error(),
                    err.debug()
                        .unwrap_or_else(|| String::from("No debug info").into())
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(State::Null)
        .expect("Failed to set pipeline to Null");
}

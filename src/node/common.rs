use opentelemetry::propagation::{Extractor, Injector};
use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::{
    logs::LoggerProvider,
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider},
    runtime,
    trace::TracerProvider,
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use tokio;
// use tracing_core::Level;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub trait Metric {
    fn collect_metrics(&self) -> Option<Vec<(String, String)>> {
        None
    }
}

pub trait Message {
    async fn next(&mut self) -> Option<&mut Self>;
    fn ser(&self) -> String;
    fn deser(&self, msg: &String) -> Self;
}

// #[tracing::instrument]
pub fn logger(message: String) {
    tokio::spawn(async move {
        // println!("{}", message);
        tracing::info!(message);
        // tracing::debug!(message);
    });
}

#[allow(dead_code)]
pub struct SessionInfo {
    pub zid: String,
    pub routers_zid: Vec<String>,
    pub peers_zid: Option<Vec<String>>,
}

pub struct DB {
    pub conn: Connection,
}

impl DB {
    pub fn new() -> Self {
        let conn = Connection::open("state.db").unwrap();
        // let conn = Connection::open_in_memory()?;

        match conn.execute(
            "CREATE TABLE state (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            topic TEXT NOT NULL
        )",
            (),
        ) {
            Ok(_) => println!("Table created!"),
            Err(_) => println!("Table already exists!"),
        }
        Self { conn }
    }
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
        ],
        SCHEMA_URL,
    )
}

#[allow(dead_code)]
fn init_meteric_provider() -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_temporality(opentelemetry_sdk::metrics::Temporality::default())
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        // .with_interval(std::time::Duration::from_secs(1))
        .with_interval(std::time::Duration::from_millis(1))
        .build();

    // let stdout_reader = PeriodicReader::builder(
    //     opentelemetry_stdout::MetricExporter::default(),
    //     runtime::Tokio,
    // )
    // .build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource())
        .with_reader(reader)
        // .with_reader(stdout_reader)
        .build();

    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

#[allow(dead_code)]
fn init_log_provider() -> LoggerProvider {
    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let log_provider = LoggerProvider::builder()
        .with_resource(resource())
        // .with_log_processor(
        //     opentelemetry_sdk::logs::BatchLogProcessor::builder(log_exporter, runtime::Tokio)
        //         .build(),
        // )
        .with_batch_exporter(log_exporter, runtime::Tokio)
        .build();

    log_provider
}

#[allow(dead_code)]
fn init_tracer_provider() -> TracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let trace_provide = TracerProvider::builder()
        .with_resource(resource())
        .with_batch_exporter(exporter, runtime::Tokio)
        .build();

    global::set_tracer_provider(trace_provide.clone());

    trace_provide
}

#[allow(dead_code)]
pub fn init_tracing_subscriber() -> OtelGuard {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer_provider = init_tracer_provider();
    let meter_provider = init_meteric_provider();
    let log_provider = init_log_provider();

    let tracer = tracer_provider.tracer("tracing-otel-subscriber");

    tracing_subscriber::registry()
        // .with(tracing_subscriber::filter::LevelFilter::from_level(
        //     Level::INFO,
        // ))
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer))
        .with(
            opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(
                &log_provider.clone(),
            ),
        )
        .init();

    OtelGuard {
        tracer_provider,
        meter_provider,
    }
}

pub struct OtelGuard {
    tracer_provider: TracerProvider,
    meter_provider: SdkMeterProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpannedMessage {
    pub message: String,
    pub span: PropagationContext,
}

pub fn spanned_message(message: String, span: tracing::Span) -> String {
    span.in_scope(|| {
        let propagation_context = PropagationContext::inject(&span.context());
        let new_message = SpannedMessage {
            message,
            span: propagation_context,
        };
        let serialized = serde_json::to_string(&new_message).unwrap();
        return serialized;
    })
}

pub fn unspanned_message(
    message: String,
) -> Result<(String, PropagationContext), serde_json::Error> {
    let serialized: SpannedMessage = serde_json::from_str(&message).unwrap();
    Ok((serialized.message, serialized.span))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropagationContext(pub HashMap<String, String>);
impl PropagationContext {
    fn empty() -> Self {
        Self(HashMap::new())
    }
    pub fn inject(context: &opentelemetry::Context) -> Self {
        global::get_text_map_propagator(|propagator| {
            let mut propagation_context = PropagationContext::empty();
            propagator.inject_context(context, &mut propagation_context);
            propagation_context
        })
    }
    pub fn extract(&self) -> opentelemetry::Context {
        global::get_text_map_propagator(|propagator| {
            return propagator.extract(self);
        })
    }
}

impl Injector for PropagationContext {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }
}

impl Extractor for PropagationContext {
    fn get(&self, key: &str) -> Option<&str> {
        let key = key.to_owned();
        self.0.get(&key).map(|v| v.as_ref())
    }
    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_ref()).collect()
    }
}

use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_sdk::{
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider},
    runtime,
    trace::TracerProvider,
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio;
use tracing::info;
use tracing_core::Level;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub trait Message {
    async fn next(&mut self) -> Option<&mut Self>
    where
        Self: Sized;
    fn ser(&self) -> String
    where
        Self: Serialize,
    {
        let serialized = serde_json::to_string(&self).unwrap();
        return serialized;
    }
    fn deser(&self, msg: &String) -> Self
    where
        Self: for<'de> Deserialize<'de> + Debug,
    {
        let deserialized: Self = serde_json::from_str(&msg).unwrap();
        return deserialized;
    }
}

// #[tracing::instrument]
pub fn logger(message: String) {
    tokio::spawn(async move {
        println!("{}", message);
        // info!(message);
    });
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

fn init_meter_provider() -> SdkMeterProvider {
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

fn init_tracer_provider() -> TracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    TracerProvider::builder()
        .with_resource(resource())
        .with_batch_exporter(exporter, runtime::Tokio)
        .build()
}

pub fn init_tracing_subscriber() -> OtelGuard {
    let tracer_provider = init_tracer_provider();
    let meter_provider = init_meter_provider();

    let tracer = tracer_provider.tracer("tracing-otel-subscriber");

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer))
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
struct SpannedMessage {
    message: String,
    span: PropagationContext,
}

use tracing_opentelemetry::OpenTelemetrySpanExt;
pub fn spanned_message(message: String) -> String {
    let parent_context = tracing::Span::current().context();
    let propagation_context = PropagationContext::inject(&parent_context);
    let new_message = SpannedMessage {
        message,
        span: propagation_context,
    };
    let serialized = serde_json::to_string(&new_message).unwrap();
    return serialized;
}
pub fn unspanned_message(
    message: String,
) -> Result<(String, PropagationContext), serde_json::Error> {
    let serialized: SpannedMessage = serde_json::from_str(&message).unwrap();
    Ok((serialized.message, serialized.span))
}

use opentelemetry::propagation::{Extractor, Injector};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropagationContext(HashMap<String, String>);
impl PropagationContext {
    fn empty() -> Self {
        let mut new = HashMap::new();
        new.insert(
            "traceparent".to_string(),
            "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".to_string(),
        );
        Self(new)
    }

    pub fn inject(context: &opentelemetry::Context) -> Self {
        global::get_text_map_propagator(|propagator| {
            let mut propagation_context = PropagationContext::empty();
            propagator.inject_context(context, &mut propagation_context);
            println!("----------------{:?}", propagation_context);
            propagation_context
        })
    }

    pub fn extract(&self) -> opentelemetry::Context {
        global::get_text_map_propagator(|propagator| propagator.extract(self))
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

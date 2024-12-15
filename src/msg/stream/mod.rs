use crate::node::common::PropagationContext;
use serde::{Deserialize, Serialize};
// use serde_json;
use crate::node::common::Message;
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stream {
    pub start: Option<u32>,
    pub length: Option<u32>,
    pub num: u32,
}

impl Stream {
    #[allow(dead_code)]
    // #[tracing::instrument]
    pub fn new(start: Option<u32>, length: Option<u32>) -> Self {
        if let (Some(start), Some(length)) = (start, length) {
            Stream {
                start: Some(start),
                length: Some(length),
                num: start as u32,
            }
        } else {
            Stream {
                start: Some(0),
                length: Some(10),
                num: 0,
            }
        }
    }
}

impl Message for Stream {
    #[tracing::instrument]
    async fn next(&mut self) -> Option<&mut Self> {
        self.num += 1;
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        // tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
        if (self.num - self.start.unwrap()) < self.length.unwrap() {
            tracing::info!(
                monotonic_counter.stream = self.num,
                "updating the Stream value",
            );
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserMessage {
    pub number: String,
    pub value: String,
    pub count: u32,
    pub bytes: Vec<u8>,
}

impl Message for UserMessage {
    #[tracing::instrument]
    async fn next(&mut self) -> Option<&mut Self> {
        self.number = (self.number.parse::<u32>().unwrap() + 1).to_string();
        self.value = format!("value {}", self.number);
        self.count += 1;
        self.bytes = self.bytes.iter().map(|x| x + 1).collect();
        if self.count > 20 {
            None
        } else {
            Some(self)
        }
    }
}

// use opentelemetry::{propagation::TextMapPropagator, trace::TraceContextExt};
// use opentelemetry_sdk::propagation::TraceContextPropagator;
// use std::collections::HashMap;
use tracing_opentelemetry::OpenTelemetrySpanExt;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineMessage {
    pub message: String,
    pub count: u32,
    pub span: PropagationContext,
    // span: HashMap<String, String>,
    // span: tracing::Span,
}

impl Default for MachineMessage {
    // #[tracing::instrument]
    fn default() -> Self {
        // let parent_context = tracing::Span::current().context();
        let parent_context = tracing::Span::none().context();
        let propagation_context = PropagationContext::inject(&parent_context);
        // let mut carrier = HashMap::new();
        // let propagator = TraceContextPropagator::new();
        // let parent_context = propagator.extract(&carrier);
        // println!("Parent context: {:?}", parent_context);
        MachineMessage {
            message: "message 0".to_string(),
            count: 0,
            span: propagation_context,
        }
    }
}

impl Message for MachineMessage {
    #[tracing::instrument]
    async fn next(&mut self) -> Option<&mut Self> {
        self.count += 1;
        self.message = format!("message {}", self.count);
        let span = tracing::Span::current();
        println!("Span: ------------------>{:?}", span);
        if self.count > 0 {
            None
        } else {
            Some(self)
        }
        // Some(self)
    }
}

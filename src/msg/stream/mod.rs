use crate::node::common::Message;
use metrics_macros::Metrics;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Metrics, Default, Serialize, Deserialize)]
pub struct Stream {
    pub start: Option<u32>,
    pub length: Option<u32>,
    pub num_metric: u32,
}

impl Stream {
    #[allow(dead_code)]
    pub fn new(start: Option<u32>, length: Option<u32>) -> Self {
        if let (Some(start), Some(length)) = (start, length) {
            Stream {
                start: Some(start),
                length: Some(length),
                num_metric: start as u32,
            }
        } else {
            Stream {
                start: Some(0),
                length: Some(10),
                num_metric: 0,
            }
        }
    }
}

impl Message for Stream {
    // #[tracing::instrument]
    async fn next(&mut self) -> Option<&mut Self> {
        self.num_metric += 2;
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        // tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
        if (self.num_metric - self.start.unwrap()) < self.length.unwrap() {
            tracing::info!(
                monotonic_counter.stream = self.num_metric,
                "updating the Stream value",
            );
            // tracing::error!(
            //     "..........in..next..........Metric: {:?}",
            //     self.collect_metrics()
            // );
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
    // #[tracing::instrument]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineMessage {
    pub message: String,
    pub count: u32,
}

impl MachineMessage {
    #[allow(dead_code)]
    pub fn new(message: String, count: u32) -> Self {
        MachineMessage { message, count }
    }
}

impl Default for MachineMessage {
    fn default() -> Self {
        MachineMessage {
            message: "message 0".to_string(),
            count: 0,
        }
    }
}

impl Message for MachineMessage {
    // #[tracing::instrument]
    async fn next(&mut self) -> Option<&mut Self> {
        self.count += 1;
        self.message = format!("message {}", self.count);
        // std::thread::sleep(std::time::Duration::from_secs(1));
        if self.count > 10 {
            None
        } else {
            Some(self)
        }
        // Some(self)
    }
}

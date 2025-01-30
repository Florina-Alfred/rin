use crate::node::common::{Message, Metric};
// use crate::node::common::{Message, Metric};
use rin_macros::{Messages, Metrics};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Metrics, Default, Serialize, Deserialize)]
pub struct SimpleMessage {
    pub start: u32,
    pub length: u32,
    pub stream_num_metric: u32,
    pub stream_test_1_metric: u32,
    pub stream_test_2_metric: u32,
}

impl SimpleMessage {
    #[allow(dead_code)]
    pub fn new(start: Option<u32>, length: Option<u32>) -> Self {
        if let (Some(start), Some(length)) = (start, length) {
            SimpleMessage {
                start,
                length,
                stream_num_metric: start as u32,
                stream_test_1_metric: 0,
                stream_test_2_metric: 0,
            }
        } else {
            SimpleMessage {
                start: 0,
                length: 10,
                stream_num_metric: 0,
                stream_test_1_metric: 0,
                stream_test_2_metric: 0,
            }
        }
    }
}

impl Message for SimpleMessage {
    // #[tracing::instrument]
    async fn next(&mut self) -> Option<&mut Self> {
        self.stream_num_metric += 1;
        self.stream_test_1_metric += 2;
        self.stream_test_2_metric += self.stream_test_1_metric - self.stream_num_metric;
        // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        // tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
        if (self.stream_num_metric - self.start) < self.length {
            // tracing::info!(
            //     monotonic_counter.stream_num = self.stream_num_metric,
            //     // monotonic_counter.stream_num = self.stream_num_metric,
            //     "updating the SimpleMessage value",
            // );
            // tracing::error!(
            //     "..........in..next..........Metric: {:?}",
            //     self.collect_metrics()
            // );
            Some(self)
        } else {
            None
        }
    }

    fn ser(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn deser(&self, msg: &String) -> Self {
        serde_json::from_str(&msg).unwrap()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Metrics)]
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

    fn ser(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn deser(&self, msg: &String) -> Self {
        serde_json::from_str(&msg).unwrap()
    }
}

// #[derive(Debug, Clone, Metrics, Messages)]
#[derive(Debug, Clone, Serialize, Deserialize, Metrics, Messages)]
// #[derive(Debug, Clone, Serialize, Deserialize, Metrics)]
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

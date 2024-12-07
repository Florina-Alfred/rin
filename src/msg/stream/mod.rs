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
    async fn next(&mut self) -> Option<&mut Self> {
        self.num += 1;
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        // tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
        if (self.num - self.start.unwrap()) < self.length.unwrap() {
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
    async fn next(&mut self) -> Option<&mut Self> {
        self.number = (self.number.parse::<u32>().unwrap() + 1).to_string();
        self.value = format!("value {}", self.number);
        self.count += 1;
        self.bytes = self.bytes.iter().map(|x| x + 1).collect();
        if self.count > 10 {
            None
        } else {
            Some(self)
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MachineMessage {
    pub message: String,
    pub count: u32,
}

impl Message for MachineMessage {
    async fn next(&mut self) -> Option<&mut Self> {
        self.message = format!("message {}", self.count);
        self.count += 1;
        Some(self)
    }
}

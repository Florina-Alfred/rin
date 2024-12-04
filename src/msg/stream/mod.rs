use serde::{Deserialize, Serialize};
// use serde_json;
use crate::node::common::Message;
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stream {
    pub start: Option<u32>,
    pub num: u32,
}

impl Stream {
    #[allow(dead_code)]
    pub fn new(start: Option<u32>) -> Self {
        if let Some(start) = start {
            Stream {
                start: Some(start),
                num: start as u32,
            }
        } else {
            Stream {
                start: None,
                num: 0,
            }
        }
    }
}

impl Message for Stream {
    fn next(&mut self) -> Option<&mut Self> {
        self.num += 5;
        Some(self)
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
    fn next(&mut self) -> Option<&mut Self> {
        self.number = (self.number.parse::<u32>().unwrap() + 1).to_string();
        self.value = format!("value {}", self.number);
        self.count += 1;
        self.bytes = self.bytes.iter().map(|x| x + 1).collect();
        Some(self)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MachineMessage {
    pub message: String,
    pub count: u32,
}

impl Message for MachineMessage {
    fn next(&mut self) -> Option<&mut Self> {
        self.message = format!("message {}", self.count);
        self.count += 1;
        Some(self)
    }
}

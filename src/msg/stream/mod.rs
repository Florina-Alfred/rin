use serde::{Deserialize, Serialize};
// use serde_json;
use crate::node::common::Message;
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stream {
    start: Option<u32>,
    num: u32,
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
        self.count += 1;
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
        self.count += 1;
        Some(self)
    }
}

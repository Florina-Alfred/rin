use crate::node::common::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StringMessage {
    pub data: String,
}

impl StringMessage {
    #[allow(dead_code)]
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl Message for StringMessage {}

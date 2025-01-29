use crate::node::common::{Message, Metric};
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

impl Message for StringMessage {
    async fn next(&mut self) -> Option<&mut Self> {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Some(self)
    }

    fn ser(&self) -> String {
        self.data.clone()
    }

    fn deser(&self, msg: &String) -> Self {
        Self { data: msg.clone() }
    }
}

impl Metric for StringMessage {}

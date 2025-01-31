use crate::node::common::{Message, Metric};
use base64::Engine;
use prost::Message as ProstMessage;
use std::io::Cursor;

pub mod simple_message {
    include!(concat!(env!("OUT_DIR"), "/simple_message.rs"));
}

pub mod lidar_data {
    include!(concat!(env!("OUT_DIR"), "/lidar_data.rs"));
}

pub use lidar_data::LidarData;
pub use simple_message::SimpleMessage;

impl Message for SimpleMessage {
    async fn next(&mut self) -> Option<&mut Self> {
        self.stream_num_metric += 1;
        self.stream_test_1_metric += 2;
        self.stream_test_2_metric += self.stream_test_1_metric - self.stream_num_metric;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        if (self.stream_num_metric - self.start) < self.length {
            Some(self)
        } else {
            None
        }
    }

    fn ser(&self) -> String {
        let mut buf = Vec::new();
        buf.reserve(self.encoded_len());
        self.encode(&mut buf).unwrap();
        base64::prelude::BASE64_STANDARD.encode(&buf)
    }

    fn deser(&self, msg: &String) -> Self {
        let bytes = base64::prelude::BASE64_STANDARD.decode(msg).unwrap();
        SimpleMessage::decode(&mut Cursor::new(bytes)).unwrap()
    }
}

impl Metric for SimpleMessage {
    fn collect_metrics(&self) -> Option<Vec<(String, String)>> {
        None
    }
}

impl Message for LidarData {
    async fn next(&mut self) -> Option<&mut Self> {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Some(self)
    }

    fn ser(&self) -> String {
        let mut buf = Vec::new();
        buf.reserve(self.encoded_len());
        self.encode(&mut buf).unwrap();
        base64::prelude::BASE64_STANDARD.encode(&buf)
    }

    fn deser(&self, msg: &String) -> Self {
        let bytes = base64::prelude::BASE64_STANDARD.decode(msg).unwrap();
        LidarData::decode(&mut Cursor::new(bytes)).unwrap()
    }
}

impl Metric for LidarData {
    fn collect_metrics(&self) -> Option<Vec<(String, String)>> {
        None
    }
}

use crate::node::common::{Message, Metric};
use base64::Engine;
use prost::Message as ProstMessage;
use std::io::Cursor;

pub mod simple_message {
    // include!(concat!(env!("OUT_DIR"), "/SimpleMessage.rs"));
    include!(concat!(env!("OUT_DIR"), "/simple_message.rs"));
}

pub use simple_message::SimpleMessage;
// pub use simple_message::InputRequest;

impl Message for SimpleMessage {
    async fn next(&mut self) -> Option<&mut Self> {
        self.stream_num_metric += 1;
        self.stream_test_1_metric += 2;
        self.stream_test_2_metric += self.stream_test_1_metric - self.stream_num_metric;
        // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
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

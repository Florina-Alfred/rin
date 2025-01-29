mod node;

use base64::Engine;
use node::common::Message;
use prost::Message as ProstMessage;
use std::io::Cursor;

pub mod report {
    include!(concat!(env!("OUT_DIR"), "/report.rs"));
}

use report::InputRequest;

impl Message for InputRequest {
    async fn next(&mut self) -> Option<&mut Self> {
        self.last_edited += 2.0;
        self.total_edits += 1;
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
        InputRequest::decode(&mut Cursor::new(bytes)).unwrap()
    }
}

#[tokio::main]
async fn main() {
    let request = String::from("Hello, World!");

    let mut local_copy = InputRequest {
        name: request.clone(),
        last_edited: 1.0,
        total_edits: 1,
    };
    println!("Original: {:?}", local_copy);

    local_copy.next().await;
    local_copy.next().await;
    local_copy.next().await;
    println!("Update: {:?}", local_copy);

    let buf = local_copy.ser();
    println!("Ser: {:?}", buf);

    let deser = local_copy.deser(&buf);
    println!("Deser: {:?}", deser);
}

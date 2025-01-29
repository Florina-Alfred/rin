use prost::Message as ProstMessage;
use std::io::Cursor;

pub mod report {
    include!(concat!(env!("OUT_DIR"), "/report.rs"));
}

trait InputRequestTrait {
    fn next(&mut self);
    fn ser(&self) -> Vec<u8>;
    fn deser(buf: &[u8]) -> Self;
}

impl InputRequestTrait for report::InputRequest {
    fn next(&mut self) {
        self.last_edited += 2.0;
        self.total_edits += 1;
    }

    fn ser(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let local_copy = report::InputRequest {
            name: self.name.clone(),
            last_edited: self.last_edited,
            total_edits: self.total_edits,
        };
        buf.reserve(local_copy.encoded_len());
        local_copy.encode(&mut buf).unwrap();
        buf
    }

    fn deser(buf: &[u8]) -> Self {
        report::InputRequest::decode(&mut Cursor::new(buf)).unwrap()
    }
}

fn main() {
    let request = String::from("Hello, World!");

    let mut local_copy = report::InputRequest {
        name: request.clone(),
        last_edited: 1.0,
        total_edits: 1,
    };
    println!("Original: {:?}", local_copy);

    local_copy.next();
    local_copy.next();
    local_copy.next();
    println!("Update: {:?}", local_copy);

    let buf = local_copy.ser();
    println!("Ser: {:?}", buf);

    let deser = report::InputRequest::deser(&buf);
    println!("Deser: {:?}", deser);
}

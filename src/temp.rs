use prost::Message;
use std::io::Cursor;

pub mod report {
    include!(concat!(env!("OUT_DIR"), "/report.rs"));
}

pub fn create_input_request(name: String) -> report::InputRequest {
    let mut input_request = report::InputRequest::default();
    input_request.name = name;
    input_request
}

pub fn serialize_report(input: &report::InputRequest) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(input.encoded_len());
    input.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_report(buf: &[u8]) -> Result<report::InputRequest, prost::DecodeError> {
    report::InputRequest::decode(&mut Cursor::new(buf))
}

fn main() -> Result<(), prost::DecodeError> {
    let request = String::from("Hello, World!");

    let report_request = create_input_request(request);
    let request_vector = serialize_report(&report_request);

    let request_deserialized_result = match deserialize_report(&request_vector) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(e) => return Err(e),
    };
    println!("{:#?}", request_deserialized_result);
    Ok(())
}

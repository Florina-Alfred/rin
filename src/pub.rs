mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use msg::stream::{MachineMessage, Stream, UserMessage};
use node::common::Message;
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // let mut stream_struct = Stream::new(Some(args.start));
    // let mut stream_struct = UserMessage {
    //     number: "1".to_string(),
    //     value: "value".to_string(),
    //     count: 0,
    //     bytes: vec![0, 1, 2, 3, 4],
    // };
    let mut stream_struct = MachineMessage {
        message: "message".to_string(),
        count: 0,
    };

    for _ in 0..50 {
        stream_struct.next().unwrap();
        println!("Stream: {:?}", stream_struct);
        node::publish(
            args.key_expr.as_str(),
            stream_struct.clone(),
            None,
            args.mode.as_str(),
            args.endpoints.iter().map(|x| x.as_str()).collect(),
        )
        .await;
    }
}

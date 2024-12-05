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

    let mut pub_msg_struct = Stream::new(Some(args.start));

    // let mut pub_msg_struct = UserMessage {
    //     number: "1".to_string(),
    //     value: "value".to_string(),
    //     count: 0,
    //     bytes: vec![0, 1, 2, 3, 4],
    // };

    // let mut pub_msg_struct = MachineMessage {
    //     message: "message".to_string(),
    //     count: 0,
    // };

    println!("Current Message: {:?}", pub_msg_struct);
    node::publish(
        args.key_expr.as_str(),
        pub_msg_struct.clone(),
        None,
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await;
    pub_msg_struct.next().unwrap();
}

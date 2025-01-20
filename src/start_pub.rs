mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
#[allow(unused_imports)]
use msg::stream::{MachineMessage, Stream, UserMessage};
use node::common;
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _guard = common::init_tracing_subscriber();

    // let pub_msg_struct = Stream::new(Some(args.start), Some(10));
    // let pub_msg_struct = MachineMessage::default();
    let pub_msg_struct = MachineMessage::new("message 0".to_string(), 0);
    node::start_publisher(
        "test_publisher",
        args.input_key_expr.as_str(),
        pub_msg_struct.clone(),
        None,
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await;
}

mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
#[allow(unused_imports)]
use msg::stream::{MachineMessage, Stream, UserMessage};
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let publisher = node::Publisher::new(
        args.key_expr.as_str(),
        args.mode.as_str(),
        None,
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();

    let pub_msg_struct = Stream::new(Some(args.start));
    println!("-------Current Message: {:?}", pub_msg_struct);
    publisher.publish(pub_msg_struct.clone()).await;
    publisher.publish(pub_msg_struct.clone()).await;
    publisher.publish(pub_msg_struct.clone()).await;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let pub_msg_struct = UserMessage {
        number: "1".to_string(),
        value: "value".to_string(),
        count: 0,
        bytes: vec![0, 1, 2, 3, 4],
    };
    println!("-------Current Message: {:?}", pub_msg_struct);
    publisher.publish(pub_msg_struct.clone()).await;
    publisher.publish(pub_msg_struct.clone()).await;
    publisher.publish(pub_msg_struct.clone()).await;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let pub_msg_struct = MachineMessage {
        message: "message".to_string(),
        count: 0,
    };
    println!("-------Current Message: {:?}", pub_msg_struct);
    publisher.publish(pub_msg_struct.clone()).await;
    publisher.publish(pub_msg_struct.clone()).await;
    publisher.publish(pub_msg_struct.clone()).await;

    // node::start_publisher(
    //     args.key_expr.as_str(),
    //     pub_msg_struct.clone(),
    //     None,
    //     args.mode.as_str(),
    //     args.endpoints.iter().map(|x| x.as_str()).collect(),
    // )
    // .await;
}

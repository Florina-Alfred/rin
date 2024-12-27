mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
#[allow(unused_imports)]
use msg::stream::{MachineMessage, Stream, UserMessage};
use tokio;
use tracing::info;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let publisher = node::Publisher::new(
        args.input_key_expr.as_str(),
        args.mode.as_str(),
        None,
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();

    let pub_msg_struct = Stream::new(Some(args.start), Some(0));
    println!("-------Current Message: {:?}", pub_msg_struct);
    for _ in 0..3 {
        publisher.publish(pub_msg_struct.clone()).await;
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let pub_msg_struct = UserMessage {
        number: "1".to_string(),
        value: "value".to_string(),
        count: 0,
        bytes: vec![0, 1, 2, 3, 4],
    };
    println!("-------Current Message: {:?}", pub_msg_struct);
    for _ in 0..3 {
        publisher.publish(pub_msg_struct.clone()).await;
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let pub_msg_struct = MachineMessage {
        message: "message".to_string(),
        count: 0,
        span: tracing::Span::current().context(),
    };
    println!("-------Current Message: {:?}", pub_msg_struct);
    for _ in 0..3 {
        publisher.publish(pub_msg_struct.clone()).await;
    }
}

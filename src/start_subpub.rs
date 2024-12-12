mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use msg::stream::{MachineMessage, Stream, UserMessage};
use tokio;
use tracing::info;

#[allow(dead_code)]
fn stream_modifier(input: Stream) -> UserMessage {
    // println!(
    // "Stream callback:- Start: {:?} Num: {:?}",
    // input.start, input.num
    // );
    let output = UserMessage {
        number: input.num.to_string(),
        value: format!("value {}", input.num),
        count: input.num,
        bytes: vec![0, 1, 2, 3, 4],
    };
    // println!("Output----------> {:?}", output);
    println!();
    return output;
}

#[allow(dead_code)]
fn user_message_modifier(input: UserMessage) -> MachineMessage {
    println!("User message callback");
    println!("Number: {}", input.number);
    println!("Value: {}", input.value);
    println!("Count: {}", input.count);
    println!("Bytes: {:?}", input.bytes);
    println!();
    MachineMessage {
        message: format!("message {}", input.number),
        count: input.count,
    }
}

#[allow(dead_code)]
fn machine_message_modifier(input: MachineMessage) -> Stream {
    println!("Machine message callback");
    println!("Message: {}", input.message);
    println!("Count: {}", input.count);
    println!();
    Stream {
        start: Some(0),
        length: Some(10),
        num: 0,
    }
}

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

    info!("Starting subscriber");
    node::start_subscriber_publisher(
        args.output_key_expr.as_str(),
        args.input_key_expr.as_str(),
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
        stream_modifier,
    )
    .await;
}

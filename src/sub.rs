mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use msg::stream::{MachineMessage, SimpleMessage, UserMessage};
use tokio;

#[allow(dead_code)]
fn generic_callback<T: std::fmt::Debug>(input: T) {
    println!("Generic callback: {:?}", input);
}

#[allow(dead_code)]
fn stream_callback(input: SimpleMessage) {
    println!(
        "SimpleMessage callback:- Start: {:?} Num: {:?}",
        input.start, input.stream_num_metric
    );
    println!();
}

#[allow(dead_code)]
fn user_message_callback(input: UserMessage) {
    println!("User message callback");
    println!("Number: {}", input.number);
    println!("Value: {}", input.value);
    println!("Count: {}", input.count);
    println!("Bytes: {:?}", input.bytes);
    println!();
}

#[allow(dead_code)]
fn machine_message_callback(input: MachineMessage) {
    println!("Machine message callback");
    println!("Message: {}", input.message);
    println!("Count: {}", input.count);
    println!();
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

    let subscriber = node::Subscriber::new(
        args.output_key_expr.as_str(),
        args.mode.as_str(),
        SimpleMessage::default(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();
    for _ in 0..3 {
        let msg = subscriber.receive_msg().await.unwrap();
        println!(
            "SimpleMessage Received message:- Start: {:?} Num: {:?}",
            msg.start, msg.stream_num_metric
        );
    }

    let subscriber = node::Subscriber::new(
        args.output_key_expr.as_str(),
        args.mode.as_str(),
        UserMessage::default(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();
    for _ in 0..3 {
        let msg = subscriber.receive_msg().await.unwrap();
        println!(
            "UserMessage Received message:- Number: {} Value: {} Count: {} Bytes: {:?}",
            msg.number, msg.value, msg.count, msg.bytes
        );
    }

    let subscriber = node::Subscriber::new(
        args.output_key_expr.as_str(),
        args.mode.as_str(),
        MachineMessage::default(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();
    for _ in 0..3 {
        let msg = subscriber.receive_msg().await.unwrap();
        println!(
            "MachineMessage Received message:- Message: {} Count: {}",
            msg.message, msg.count
        );
    }
}

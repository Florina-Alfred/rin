mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use msg::stream::{MachineMessage, Stream, UserMessage};
use node::common;
use tokio;

#[allow(dead_code)]
#[tracing::instrument]
fn generic_callback<T: std::fmt::Debug>(input: T) {
    println!("Generic callback: {:?}", input);
}

#[allow(dead_code)]
#[tracing::instrument]
fn stream_callback(input: Stream) {
    println!(
        "Stream callback:- Start: {:?} Num: {:?}",
        input.start, input.num
    );
    if input.num == 2025 {
        tracing::warn!("Happy new year!");
    }
    println!();
}

#[allow(dead_code)]
#[tracing::instrument]
fn user_message_callback(input: UserMessage) {
    println!("User message callback");
    println!("Number: {}", input.number);
    println!("Value: {}", input.value);
    println!("Count: {}", input.count);
    println!("Bytes: {:?}", input.bytes);
    println!();
}

#[allow(dead_code)]
#[tracing::instrument]
fn machine_message_callback(input: MachineMessage) {
    tracing::debug_span!("Machine message callback");
    tracing::debug_span!("Message: {}", input.message);
    tracing::debug_span!("Count: {}", input.count);
    println!();
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _guard = common::init_tracing_subscriber();

    node::start_subscriber(
        "test_subscriber",
        args.output_key_expr.as_str(),
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
        vec![
            // generic_callback,
            // generic_callback,
            // stream_callback,
            stream_callback,
            // user_message_callback,
            // user_message_callback,
            // user_message_callback,
            // machine_message_callback,
            // machine_message_callback,
        ],
    )
    .await;
}

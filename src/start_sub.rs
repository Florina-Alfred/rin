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
    tracing::warn!("Generic callback: {:?}", input);
}

#[allow(dead_code)]
#[tracing::instrument]
fn stream_callback(input: Stream) {
    tracing::info!(
        "Stream callback:- Start: {:?} Num: {:?}",
        input.start,
        input.stream_num_metric,
    );
    if input.stream_num_metric == 2025 {
        tracing::warn!("Happy new year!");
    }
}

#[allow(dead_code)]
#[tracing::instrument]
fn user_message_callback(input: UserMessage) {
    tracing::info!("User message callback");
    tracing::info!("Number: {}", input.number);
    tracing::info!("Value: {}", input.value);
    tracing::info!("Count: {}", input.count);
    tracing::info!("Bytes: {:?}", input.bytes);
}

#[allow(dead_code)]
#[tracing::instrument]
fn machine_message_callback(input: MachineMessage) {
    tracing::info!("Machine message callback");
    tracing::info!("input: {:?}", input);
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
            // stream_callback,
            // user_message_callback,
            // user_message_callback,
            // user_message_callback,
            machine_message_callback,
            machine_message_callback,
        ],
    )
    .await;
}

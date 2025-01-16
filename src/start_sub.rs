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
    tracing::warn!(
        "Stream callback:- Start: {:?} Num: {:?} with metrics: {:?}",
        input.start,
        input.num_metric,
        input.collect_metrics()
    );
    // if let Some(metrics) = input.collect_metrics() {
    //     for (key, value) in metrics {
    //         tracing::info!(key = value.as_str(), "Metric");
    //         println!("-------outside------{}: {}", key, value);
    //     }
    // }
    if input.num_metric == 2025 {
        tracing::warn!("Happy new year!");
    }
}

#[allow(dead_code)]
#[tracing::instrument]
fn user_message_callback(input: UserMessage) {
    tracing::warn!("User message callback");
    tracing::warn!("Number: {}", input.number);
    tracing::warn!("Value: {}", input.value);
    tracing::warn!("Count: {}", input.count);
    tracing::warn!("Bytes: {:?}", input.bytes);
}

#[allow(dead_code)]
#[tracing::instrument]
fn machine_message_callback(input: MachineMessage) {
    tracing::debug_span!("Machine message callback");
    tracing::debug_span!("Message: {}", input.message);
    tracing::debug_span!("Count: {}", input.count);
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

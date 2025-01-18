mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use msg::stream::{MachineMessage, Stream, UserMessage};
use node::common;
// use node::common::PropagationContext;
use tokio;
// use tracing_opentelemetry::OpenTelemetrySpanExt;

#[allow(dead_code)]
#[tracing::instrument]
fn stream_modifier(input: Stream) -> UserMessage {
    let output = UserMessage {
        number: input.stream_num_metric.to_string(),
        value: format!("value {}", input.stream_num_metric),
        count: input.stream_num_metric,
        bytes: vec![0, 1, 2, 3, 4],
    };
    println!();
    return output;
}

#[allow(dead_code)]
#[tracing::instrument]
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
#[tracing::instrument]
fn machine_message_modifier(input: MachineMessage) -> Stream {
    println!("Machine message callback");
    println!("Message: {}", input.message);
    println!("Count: {}", input.count);
    println!();
    Stream {
        start: Some(0),
        length: Some(10),
        stream_num_metric: 0,
        stream_test_1_metric: 0,
        stream_test_2_metric: 0,
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _guard = common::init_tracing_subscriber();

    node::start_subscriber_publisher(
        "test_subscriber_publisher",
        args.output_key_expr.as_str(),
        args.input_key_expr.as_str(),
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
        stream_modifier,
    )
    .await;
}

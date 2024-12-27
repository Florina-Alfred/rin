mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use msg::stream::{MachineMessage, Stream, UserMessage};
use node::common;
use tokio;
use tracing_opentelemetry::OpenTelemetrySpanExt;

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
    // let parent_context = input.span.extract("callback");
    // println!("Parent context from MM callback: {:?}", parent_context);
    // let span = tracing::Span::current();
    // span.set_parent(parent_context);
    // println!("Span inside Machine message callback {:?}", span);
    tracing::debug_span!("Machine message callback");
    tracing::debug_span!("Message: {}", input.message);
    tracing::debug_span!("Count: {}", input.count);
    // tracing::debug_span!("Span: {:?}", input.span);
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
            // stream_callback,
            // user_message_callback,
            // user_message_callback,
            // user_message_callback,
            machine_message_callback,
            // machine_message_callback,
        ],
    )
    .await;
}

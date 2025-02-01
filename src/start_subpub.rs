mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
// use msg::proto::SimpleMessage;
use msg::stream::SimpleMessage;
use msg::stream::{MachineMessage, UserMessage};
use node::common;
// use node::common::PropagationContext;
use tokio;
// use tracing_opentelemetry::OpenTelemetrySpanExt;

#[allow(dead_code)]
#[tracing::instrument]
fn stream_to_usermessage_modifier(input: SimpleMessage) -> UserMessage {
    let output = UserMessage {
        number: input.stream_num_metric.to_string(),
        value: format!("value {}", input.stream_num_metric),
        count: input.stream_num_metric,
        bytes: vec![0, 1, 2, 3, 4],
    };
    tracing::info!("SimpleMessage to UserMessage modifier: {:?}", output);
    return output;
}

#[allow(dead_code)]
#[tracing::instrument]
fn stream_to_stream_modifier(input: SimpleMessage) -> SimpleMessage {
    let output = SimpleMessage {
        start: input.stream_num_metric,
        length: 200,
        stream_num_metric: input.stream_num_metric + 1,
        stream_test_1_metric: input.stream_test_1_metric + 2,
        stream_test_2_metric: input.stream_test_2_metric + 3,
    };
    tracing::info!("SimpleMessage to UserMessage modifier: {:?}", output);
    return output;
}

#[allow(dead_code)]
#[tracing::instrument]
fn usermessage_machinemachine_modifier(input: UserMessage) -> MachineMessage {
    let output = MachineMessage {
        message: format!("message {}", 0),
        count: input.count,
    };
    tracing::info!("User message callback:- {:?}", input);
    return output;
}

#[allow(dead_code)]
#[tracing::instrument]
fn machinemessage_stream_modifier(input: MachineMessage) -> SimpleMessage {
    let output = SimpleMessage {
        start: input.count,
        length: 200,
        stream_num_metric: input.count,
        stream_test_1_metric: 0,
        stream_test_2_metric: 0,
    };
    tracing::info!("Machine message callback:- {:?}", input);
    return output;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _guard = common::init_tracing_subscriber();

    node::start_subscriber_publisher(
        "test_subscriber_publisher",
        args.input_key_expr.as_str(),
        args.output_key_expr.as_str(),
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
        // stream_to_usermessage_modifier,
        stream_to_stream_modifier,
        // usermessage_machinemachine_modifier,
        // machinemessage_stream_modifier,
    )
    .await;
}

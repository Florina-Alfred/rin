mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
#[allow(unused_imports)]
use msg::stream::{MachineMessage, SimpleMessage, UserMessage};
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let trace_subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(trace_subscriber).unwrap();

    let publisher = node::Publisher::new(
        args.input_key_expr.as_str(),
        args.mode.as_str(),
        None,
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();

    let mut pub_msg_struct = SimpleMessage::new(Some(args.start), Some(0));
    for _ in 0..3 {
        pub_msg_struct.stream_num_metric += 1;
        pub_msg_struct.stream_test_1_metric += 2;
        pub_msg_struct.stream_test_2_metric +=
            pub_msg_struct.stream_test_1_metric - pub_msg_struct.stream_num_metric;
        publisher.publish(pub_msg_struct.clone()).await;
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let mut pub_msg_struct = UserMessage {
        number: "1".to_string(),
        value: "value".to_string(),
        count: 0,
        bytes: vec![0, 1, 2, 3, 4],
    };
    for _ in 0..3 {
        pub_msg_struct.number = format!("{}", pub_msg_struct.count);
        pub_msg_struct.value = format!("value {}", pub_msg_struct.count);
        pub_msg_struct.count += 1;
        pub_msg_struct.bytes = pub_msg_struct.bytes.iter().map(|x| x + 1).collect();
        publisher.publish(pub_msg_struct.clone()).await;
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let mut pub_msg_struct = MachineMessage {
        message: "message".to_string(),
        count: 0,
    };
    for _ in 0..3 {
        pub_msg_struct.message = format!("message {}", pub_msg_struct.count);
        pub_msg_struct.count += 1;
        publisher.publish(pub_msg_struct.clone()).await;
    }

    publisher.token.undeclare().await.unwrap();
}

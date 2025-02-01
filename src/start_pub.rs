mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
#[allow(unused_imports)]
// use msg::proto::{LidarData, SimpleMessage};
use msg::stream::{LidarData, SimpleMessage};
#[allow(unused_imports)]
use msg::stream::{MachineMessage, UserMessage};
use node::common;
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _guard = common::init_tracing_subscriber();

    let pub_msg_struct = SimpleMessage::new(Some(args.start), Some(10000));
    // let pub_msg_struct = SimpleMessage {
    //     start: args.start,
    //     length: 100,
    //     stream_num_metric: args.start,
    //     stream_test_1_metric: 0,
    //     stream_test_2_metric: 0,
    // };

    // let pub_msg_struct = LidarData::new(0.0, 0.0);
    // let pub_msg_struct = LidarData {
    //     counter: 0,
    //     home_x: 0.0,
    //     home_y: 0.0,
    //     lidar_data_x_history: Vec::new(),
    //     lidar_data_y_history: Vec::new(),
    // };

    // let pub_msg_struct = MachineMessage::default();
    // let pub_msg_struct = MachineMessage::new("message 0".to_string(), 0);
    // let pub_msg_struct = UserMessage {
    //     number: "0".to_string(),
    //     value: "value 0".to_string(),
    //     count: 0,
    //     bytes: vec![0, 1, 2, 3, 4],
    // };
    node::start_publisher(
        "test_publisher",
        args.input_key_expr.as_str(),
        pub_msg_struct.clone(),
        None,
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await;
}

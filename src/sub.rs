mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use msg::stream::{MachineMessage, Stream, UserMessage};
use tokio;

// fn generic_callback<T: std::fmt::Debug>(input: T) {
//     println!("Generic callback: {:?}", input);
// }

#[allow(dead_code)]
fn stream_callback(input: Stream) {
    println!(
        "Stream callback:- Start: {:?} Num: {:?}",
        input.start, input.num
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

    let subscriber = node::Subscriber::new(
        args.key_expr.as_str(),
        args.mode.as_str(),
        Stream::default(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "Stream Received message:- Start: {:?} Num: {:?}",
        msg.start, msg.num
    );
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "Stream Received message:- Start: {:?} Num: {:?}",
        msg.start, msg.num
    );
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "Stream Received message:- Start: {:?} Num: {:?}",
        msg.start, msg.num
    );

    let subscriber = node::Subscriber::new(
        args.key_expr.as_str(),
        args.mode.as_str(),
        UserMessage::default(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "UserMessage Received message:- Number: {} Value: {} Count: {} Bytes: {:?}",
        msg.number, msg.value, msg.count, msg.bytes
    );
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "UserMessage Received message:- Number: {} Value: {} Count: {} Bytes: {:?}",
        msg.number, msg.value, msg.count, msg.bytes
    );
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "UserMessage Received message:- Number: {} Value: {} Count: {} Bytes: {:?}",
        msg.number, msg.value, msg.count, msg.bytes
    );

    let subscriber = node::Subscriber::new(
        args.key_expr.as_str(),
        args.mode.as_str(),
        MachineMessage::default(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await
    .unwrap();
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "MachineMessage Received message:- Message: {} Count: {}",
        msg.message, msg.count
    );
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "MachineMessage Received message:- Message: {} Count: {}",
        msg.message, msg.count
    );
    let msg = subscriber.receive_msg().await.unwrap();
    println!(
        "MachineMessage Received message:- Message: {} Count: {}",
        msg.message, msg.count
    );

    // node::start_subscriber(
    //     args.key_expr.as_str(),
    //     args.mode.as_str(),
    //     args.endpoints.iter().map(|x| x.as_str()).collect(),
    //     // generic_callback,
    //     // stream_callback,
    //     // user_message_callback,
    //     // machine_message_callback,
    //     vec![
    //         // stream_callback,
    //         // stream_callback,
    //         // user_message_callback,
    //         // user_message_callback,
    //         // user_message_callback,
    //         machine_message_callback,
    //         // machine_message_callback,
    //     ],
    // )
    // .await;
}

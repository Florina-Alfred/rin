mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
#[allow(unused_imports)]
use msg::stream::{MachineMessage, Stream, UserMessage};
use tokio;
use tracing::info;

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

    let pub_msg_struct = Stream::new(Some(args.start), Some(3));
    info!(?pub_msg_struct, "Starting publisher");
    node::start_publisher(
        args.input_key_expr.as_str(),
        pub_msg_struct.clone(),
        None,
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await;
    info!("Publisher ended");
}

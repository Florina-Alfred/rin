mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use tokio;

fn subscriber_callback<T: std::fmt::Debug>(msg: T) {
    println!("Callback print: {:?}", msg);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    node::subscribe(
        Some(args.key_expr.as_str()),
        Some(args.mode.as_str()),
        Some(args.endpoints.iter().map(|x| x.as_str()).collect()),
        Some(subscriber_callback),
    )
    .await;
}

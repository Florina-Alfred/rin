mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use msg::stream::Stream;
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let stream_struct = Stream::new(Some(args.start));
    node::publish(
        Some(args.key_expr.as_str()),
        Some(stream_struct),
        None,
        Some(args.mode.as_str()),
        Some(args.endpoints.iter().map(|x| x.as_str()).collect()),
    )
    .await;
}

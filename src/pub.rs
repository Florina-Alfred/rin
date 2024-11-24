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
        args.key_expr.as_str(),
        stream_struct,
        None,
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
    )
    .await;
}

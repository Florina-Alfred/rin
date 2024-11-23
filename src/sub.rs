mod args;
mod msg;
mod node;

// use node::z_pub;
use args::Args;
use clap::Parser;
// use msg::stream::Stream;
use tokio;

fn subscriber_callback<T: std::fmt::Debug>(msg: T) {
    println!("Callback print: {:?}", msg);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tokio::spawn(async move {
        node::subscribe(
            Some(args.key_expr.as_str()),
            Some(args.mode.as_str()),
            Some(args.endpoints.iter().map(|x| x.as_str()).collect()),
            Some(subscriber_callback),
        )
        .await;
    });

    tokio::signal::ctrl_c().await.unwrap();
}

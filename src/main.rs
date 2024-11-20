mod args;
mod msg;
mod node;

// use node::z_pub;
use args::Args;
use clap::Parser;
use msg::stream::Stream;
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
            Some(subscriber_callback::<T>),
        )
        .await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let args = Args::parse();
    let stream_struct = Stream::new(Some(0));
    tokio::spawn(async move {
        node::publish(
            Some(args.key_expr.as_str()),
            Some(stream_struct),
            None,
            Some(args.mode.as_str()),
            Some(args.endpoints.iter().map(|x| x.as_str()).collect()),
        )
        .await;
    });

    tokio::signal::ctrl_c().await.unwrap();
}

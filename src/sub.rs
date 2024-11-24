mod args;
mod msg;
mod node;

use args::Args;
use clap::Parser;
use tokio;

fn subscriber_callback(input: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(input_str) = input.downcast_ref::<String>() {
        println!("String: {}", input_str);
        let number = input_str.split("-").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        println!("Number: {}", number);
    } else if let Some(input_int) = input.downcast_ref::<i32>() {
        println!("Integer: {}", input_int);
    } else if let Some(input_float) = input.downcast_ref::<f64>() {
        println!("Float: {}", input_float);
    } else if let Some(input_bool) = input.downcast_ref::<bool>() {
        println!("Boolean: {}", input_bool);
    } else {
        println!("Unknown type {:?}", input);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    node::subscribe(
        args.key_expr.as_str(),
        args.mode.as_str(),
        args.endpoints.iter().map(|x| x.as_str()).collect(),
        subscriber_callback,
    )
    .await;
}

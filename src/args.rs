use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 0)]
    pub start: u32,

    #[arg(short, long, default_value = "demo/example/zenoh-rs-pub")]
    pub input_key_expr: String,

    #[arg(short, long, default_value = "demo/example/zenoh-rs-sub")]
    pub output_key_expr: String,

    #[arg(short, long, default_value = "client")]
    pub mode: String,

    #[arg(short, long, default_value = "tcp/0.0.0.0:7447")]
    pub endpoints: Vec<String>,
}

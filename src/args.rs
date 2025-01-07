use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Start value for the stream
    #[arg(short, long, default_value_t = 0)]
    pub start: u32,

    /// Key expression for the publisher
    #[arg(short, long, default_value = "test_topic")]
    pub input_key_expr: String,

    /// Key expression for the subscriber
    #[arg(short, long, default_value = "test_topic")]
    pub output_key_expr: String,

    /// Mode of operation
    #[arg(short, long, default_value = "client")]
    // #[arg(short, long, default_value = "peer")]
    pub mode: String,

    /// Endpoints for the publisher/subscriber
    #[arg(short, long, default_value = "tcp/0.0.0.0:7447")]
    // #[arg(short, long, default_value = "")]
    pub endpoints: Vec<String>,
}

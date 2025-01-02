mod msg;
mod node;

use clap::{Parser, Subcommand};
use msg::single::StringMessage;
use std::io::{self, BufRead};
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// name of the application
    #[arg(short, long, default_value = "pubsub")]
    project: String,

    /// subcommand
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// producer messages on TOPIC
    Producer {
        /// name of the producer
        #[arg(short, long, default_value = "producer")]
        name: String,

        /// topic to produce messages
        #[arg(short, long)]
        topic: String,

        #[arg(short, long, action = clap::ArgAction::Count, default_value = "0")]
        repeat: u8,

        #[arg(short, long, default_value = "Hello World!")]
        message: String,

        #[arg(short, long, default_value_t = 1)]
        delay: usize,

        #[arg(short, long, default_value = "peer")]
        mode: String,

        #[arg(short, long, default_value = "")]
        endpoints: Vec<String>,
    },

    /// consumer messages on TOPIC
    Subscriber {
        /// name of the subscriber
        #[arg(short, long, default_value = "subscriber")]
        name: String,

        /// topic to consume messages
        #[arg(short, long)]
        topic: String,

        #[arg(short, long, default_value = "peer")]
        mode: String,

        #[arg(short, long, default_value = "")]
        endpoints: Vec<String>,
    },
}

async fn producer_loop(
    project: &str,
    name: &str,
    topic: &str,
    mode: &str,
    endpoints: Vec<&str>,
    repeat: u8,
    message: StringMessage,
    delay: usize,
) {
    let publisher = node::Publisher::new(topic, mode, None, endpoints)
        .await
        .unwrap();
    debug!("Producer: {}", name);
    std::thread::sleep(std::time::Duration::from_secs(1));
    loop {
        if repeat > 0 {
            for _ in 0..repeat {
                info!(
                    "Producing message for project: {} at {:?} at topic: {}",
                    project, message, topic
                );
                publisher.publish(message.clone()).await;
                std::thread::sleep(std::time::Duration::from_secs(delay.try_into().unwrap()));
            }
        } else {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            eprint!("Send message: ");

            handle.read_line(&mut buffer).unwrap();
            publisher.publish(StringMessage::new(buffer.clone())).await;
            info!(
                "Producing message for project: {} at {} at topic: {}",
                project,
                buffer.trim(),
                topic
            );
        }
    }
}

async fn subscriber_loop(
    project: &str,
    name: &str,
    topic: &str,
    mode: &str,
    endpoints: Vec<String>,
) {
    let subscriber = node::Subscriber::new(
        topic,
        mode,
        StringMessage::default(),
        endpoints.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
    )
    .await
    .unwrap();

    debug!("Subscriber: {}", name);
    std::thread::sleep(std::time::Duration::from_secs(1));
    loop {
        let msg = subscriber.receive_msg().await.unwrap();
        eprintln!("Recived message: {:?}", msg.data.trim());
        info!(
            "Consuming message for project: {} at {:?} at topic: {}",
            project, msg, topic
        );

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();
    let _guard = node::common::init_tracing_subscriber();

    debug!("Args: {:?}", args.command);

    match args.command {
        Commands::Producer {
            name,
            topic,
            repeat,
            message,
            delay,
            mode,
            endpoints,
        } => {
            producer_loop(
                &args.project,
                &name,
                &topic,
                &mode,
                endpoints.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                repeat,
                StringMessage::new(message),
                delay,
            )
            .await;
        }
        Commands::Subscriber {
            name,
            topic,
            mode,
            endpoints,
        } => {
            subscriber_loop(&args.project, &name, &topic, &mode, endpoints).await;
        }
    }
}

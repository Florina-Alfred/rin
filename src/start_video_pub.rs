mod node;

use node::common;
use tokio;

#[tokio::main]
async fn main() {
    let _guard = common::init_tracing_subscriber();

    node::start_video_pub("/dev/video0", "tester").await;
}

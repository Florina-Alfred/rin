# Rin: Rust Industrial Networking Framework

Rin is a modern, high-performance Rust framework for building distributed, real-time, and streaming network applications. It provides a set of robust tools and binaries for publish/subscribe messaging, video streaming, liveliness tracking, and WebSocket communication, leveraging the Zenoh protocol, async Rust, and strong observability via tracing and metrics.

## Features
- **High-performance publish/subscribe** messaging with Zenoh
- **Video streaming** over RTSP and MJPEG
- **WebSocket server and client** for integration with web systems
- **Liveliness tracking** for monitoring distributed nodes
- **Flexible CLI tools** for rapid prototyping
- **Metrics and message macros** for auto-instrumentation and serialization
- **Async-first** and production-ready with Tokio

## Getting Started

### Prerequisites
- Rust (edition 2021, recommended latest stable)
- [GStreamer](https://gstreamer.freedesktop.org/) for video streaming binaries
- [OpenCV](https://opencv.org/) for MJPEG sample server

### Installation
Clone the repository and build the project:
```bash
git clone https://github.com/florina-Alfred/rin
cd rin
cargo build --release
```

This will build all binaries under `target/release/`.

## Binaries & Usage

Below are some of the key CLI tools provided by Rin:

| Binary     | Description                                 |
|------------|---------------------------------------------|
| `sp`       | Start a publisher (producer)                |
| `ss`       | Start a subscriber (consumer)               |
| `ssp`      | Start a sub-pub (subscriber-publisher) node |
| `p`        | Standalone publisher                        |
| `s`        | Standalone subscriber                       |
| `vp`       | Start RTSP video publisher                  |
| `vs`       | Start RTSP video subscriber                 |
| `ws`       | WebSocket server bridge                     |
| `cws`      | WebSocket client                            |
| `live`     | Liveliness subscriber                       |

### Example: Publish and Subscribe

#### Start a Subscriber
Open a new terminal and run:
```bash
./target/release/ss --output-key-expr demo/topic
```

#### Start a Publisher
Open another terminal and run:
```bash
./target/release/sp --input-key-expr demo/topic --start 42
```

#### Video Streaming
- Start video publisher:
  ```bash
  ./target/release/vp
  ```
- Start video subscriber:
  ```bash
  ./target/release/vs
  ```

#### WebSocket Bridge
- Start the WebSocket server:
  ```bash
  ./target/release/ws
  ```

## Comparison with ROS

| Feature            | Rin                               | ROS (Robot Operating System)      |
|--------------------|-----------------------------------|-----------------------------------|
| Language           | Rust                              | C++, Python, others               |
| Messaging Backend  | Zenoh                            | Custom ROS, DDS, ZeroMQ, etc.     |
| Metrics/Tracing    | Built-in with tracing/macros      | Add-ons, not default              |
| Video Streaming    | RTSP/MJPEG via GStreamer/OpenCV   | ROS topics, image_transport       |
| Liveliness         | Built-in via Zenoh                | Node heartbeat, DDS QoS           |
| WebSocket Support  | Direct binary (ws/cws)            | Via ROS Bridge, not native        |
| Async-first        | Yes (Tokio, async Rust)           | Partial (Python asyncio, DDS)     |
| Platform           | Linux, cross-platform (Rust)      | Linux, some Windows/macOS support |
| Extensibility      | Rust traits/macros                | ROS nodes/packages/plugins        |




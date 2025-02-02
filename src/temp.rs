use gstreamer::prelude::*;
use gstreamer_app::AppSink;
use opencv::core::{Mat, Vector};
use opencv::imgcodecs::{imencode, IMWRITE_JPEG_QUALITY};
use std::io::Write;
use std::net::{SocketAddr, TcpListener};

const BASE_RESPONSE: &[u8] =
    b"HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    gstreamer::init().expect("Failed to initialize GStreamer");

    let pipeline = gstreamer::Pipeline::with_name("test-pipeline");

    let rtspsrc = gstreamer::ElementFactory::make("rtspsrc")
        .name("rtspsrc")
        .property_from_str(
            "location",
            format!("rtsp://0.0.0.0:8554/{}", "tester").as_str(),
        )
        .property_from_str("latency", "0")
        .build()
        .expect("Failed to create rtspsrc element");

    let decodebin = gstreamer::ElementFactory::make("decodebin")
        .name("decodebin")
        .build()
        .expect("Failed to create decodebin element");

    let videoconvert = gstreamer::ElementFactory::make("videoconvert")
        .name("videoconvert")
        .build()
        .expect("Failed to create videoconvert element");

    let appsink = gstreamer::ElementFactory::make("appsink")
        .name("appsink")
        .property_from_str("sync", "false") // Optional: disable synchronization
        .build()
        .expect("Failed to create appsink element");

    pipeline
        .add_many(&[&rtspsrc, &decodebin, &videoconvert, &appsink])
        .expect("Failed to add elements to the pipeline");

    rtspsrc.connect_pad_added({
        let decodebin = decodebin.clone();
        move |_, pad| {
            let decodebin_pad = decodebin.static_pad("sink").unwrap();
            pad.link(&decodebin_pad)
                .expect("Failed to link rtspsrc to decodebin");
        }
    });

    decodebin.connect_pad_added({
        let videoconvert = videoconvert.clone();
        move |_, pad| {
            if let Some(sinkpad) = videoconvert.static_pad("sink") {
                pad.link(&sinkpad)
                    .expect("Failed to link decodebin to videoconvert");
            }
        }
    });

    videoconvert
        .link(&appsink)
        .expect("Failed to link videoconvert to appsink");

    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Failed to set pipeline to Playing");

    let bus = pipeline.bus().unwrap();
    let appsink = pipeline.by_name("appsink").unwrap();
    let appsink = appsink.dynamic_cast::<AppSink>().unwrap(); // Cast to AppSink

    // Setting up TCP server
    let address: SocketAddr = "127.0.0.1:8280".parse()?;
    let listener = TcpListener::bind(address)?;
    println!("Listening for connections at {address}");

    // Accept incoming HTTP connection
    let (mut stream, addr) = listener.accept()?;
    println!("Client connected: {addr}");

    // Write initial HTTP response
    stream.write_all(BASE_RESPONSE)?;

    let encode_params = opencv::core::Vector::from_slice(&[IMWRITE_JPEG_QUALITY, 70]);
    let mut buffer = Mat::default();
    let mut jpeg_buffer = opencv::core::Vector::<u8>::new(); // Buffer for JPEG data

    loop {
        match appsink.pull_sample() {
            Ok(sample) => {
                let buffer = sample.buffer().unwrap();
                let map = buffer.map_readable().unwrap();
                let raw_data = map.as_slice();

                // Convert raw data to OpenCV Mat
                let frame = opencv::core::Mat::from_bytes::<u8>(raw_data).unwrap();

                // Encode frame to JPEG
                imencode(".jpg", &frame, &mut jpeg_buffer, &encode_params).unwrap();

                // Prepare the MJPEG frame header and the encoded JPEG data
                let header = format!(
                    "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                    jpeg_buffer.len()
                );

                let packet = [header.as_bytes(), jpeg_buffer.as_slice()].concat();

                // Send the MJPEG frame to the browser
                stream.write_all(&packet)?;
            }
            Err(_) => {
                println!("No more frames to retrieve.");
                break;
            }
        }
    }

    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Failed to set pipeline to Null");

    Ok(()) // Return result properly
}


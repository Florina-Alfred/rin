use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline, State};

fn main() {
    // Initialize GStreamer
    gstreamer::init().expect("Failed to initialize GStreamer");

    // Create the GStreamer pipeline
    let pipeline = Pipeline::with_name("v4l2-rtsp-stream");

    // Create elements for the pipeline
    let v4l2src = ElementFactory::make("v4l2src")
        .name("v4l2src")
        .property_from_str("device", "/dev/video0")
        .build()
        .expect("Failed to create v4l2src element");

    let videoconvert = ElementFactory::make("videoconvert")
        .name("videoconvert")
        .build()
        .expect("Failed to create videoconvert element");

    // Create the x264 encoder with the 'tune' and 'speed-preset' properties
    let x264enc = ElementFactory::make("x264enc")
        .name("x264enc")
        .property_from_str("tune", "zerolatency") // Tune for low latency
        .property_from_str("speed-preset", "ultrafast") // Use ultrafast encoding speed
        .build()
        .expect("Failed to create x264enc element");

    // Create the RTSP client sink with transport settings
    let rtspclientsink = ElementFactory::make("rtspclientsink")
        .name("rtspclientsink")
        .property_from_str("location", "rtsp://0.0.0.0:8554/tester")
        .property_from_str("protocols", "tcp") // Set to TCP
        .build()
        .expect("Failed to create rtspclientsink element");

    // Add all elements to the pipeline
    pipeline
        .add_many(&[&v4l2src, &videoconvert, &x264enc, &rtspclientsink])
        .expect("Failed to add elements to the pipeline");

    gstreamer::Element::link_many([&v4l2src, &videoconvert, &x264enc, &rtspclientsink]).unwrap();

    // Start playing the pipeline
    pipeline
        .set_state(State::Playing)
        .expect("Failed to set pipeline to Playing");

    // Wait until an error or EOS (End Of Stream)
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        match msg.view() {
            gstreamer::MessageView::Eos(..) => {
                println!("End of Stream");
                break;
            }
            gstreamer::MessageView::Error(err) => {
                eprintln!(
                    "Error: {}: {}",
                    err.error(),
                    err.debug()
                        .unwrap_or_else(|| String::from("No debug info").into())
                );
                break;
            }
            _ => (),
        }
    }

    // Clean up the pipeline
    pipeline
        .set_state(State::Null)
        .expect("Failed to set pipeline to Null");
}

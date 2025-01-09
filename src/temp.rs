use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline, State, StateChangeError};

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

    // Create the x264 encoder without the 'preset' and 'tune' properties
    let x264enc = ElementFactory::make("x264enc")
        .name("x264enc")
        .property_from_str("bitrate", "200") // Set bitrate to 200 kbps
        .build()
        .expect("Failed to create x264enc element");

    // Create the RTSP client sink with transport settings
    let rtspclientsink = ElementFactory::make("rtspclientsink")
        .name("rtspclientsink")
        .property_from_str("location", "rtsp://0.0.0.0:8554/tester")
        .property_from_str("protocols", "tcp") // Set to TCP (you can also use 'udp' if needed)
        .build()
        .expect("Failed to create rtspclientsink element");

    // Add all elements to the pipeline
    pipeline
        .add_many(&[&v4l2src, &videoconvert, &x264enc, &rtspclientsink])
        .expect("Failed to add elements to the pipeline");

    // Link the elements together
    v4l2src
        .link(&videoconvert)
        .expect("Failed to link v4l2src to videoconvert");

    videoconvert
        .link(&x264enc)
        .expect("Failed to link videoconvert to x264enc");

    x264enc
        .link(&rtspclientsink)
        .expect("Failed to link x264enc to rtspclientsink");

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

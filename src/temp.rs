use gstreamer::prelude::*;
use gstreamer::{Bus, ElementFactory, Message, MessageView, Pipeline, State, StateChangeError};

fn main() {
    // Initialize GStreamer
    gstreamer::init().expect("Failed to initialize GStreamer");

    // Create a new GStreamer pipeline
    let pipeline = Pipeline::with_name("test-pipeline");

    // Create the elements
    let rtspsrc = ElementFactory::make("rtspsrc")
        .name("rtspsrc")
        .property_from_str("location", "rtsp://localhost:8554/tester")
        .property_from_str("latency", "0")
        .build()
        .expect("Failed to create rtspsrc element");

    let decodebin = ElementFactory::make("decodebin")
        .name("decodebin")
        .build()
        .expect("Failed to create decodebin element");

    let videoconvert = ElementFactory::make("videoconvert")
        .name("videoconvert")
        .build()
        .expect("Failed to create videoconvert element");

    let autovideosink = ElementFactory::make("autovideosink")
        .name("autovideosink")
        .build()
        .expect("Failed to create autovideosink element");

    // Add elements to the pipeline
    pipeline
        .add_many(&[&rtspsrc, &decodebin, &videoconvert, &autovideosink])
        .expect("Failed to add elements to the pipeline");

    // Link the elements
    rtspsrc
        .link(&decodebin)
        .expect("Failed to link rtspsrc to decodebin");
    decodebin.connect_pad_added(move |_, pad| {
        // When the decodebin is ready, link it to videoconvert
        if let Some(sinkpad) = videoconvert.static_pad("sink") {
            pad.link(&sinkpad)
                .expect("Failed to link decodebin to videoconvert");
        }
    });
    videoconvert
        .link(&autovideosink)
        .expect("Failed to link videoconvert to autovideosink");

    // Start playing the pipeline
    pipeline
        .set_state(State::Playing)
        .expect("Failed to set pipeline to Playing");

    // Wait until an error or EOS (End Of Stream)
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
        match msg.view() {
            MessageView::Eos(..) => {
                println!("End of Stream");
                break;
            }
            MessageView::Error(err) => {
                eprintln!(
                    "Error: {}: {}",
                    err.error(),
                    err.debug().unwrap_or_else(|| String::from("No debug info"))
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


use gstreamer::prelude::*;
use gstreamer::{ElementFactory, MessageView, Pipeline, State};
use std::sync::{Arc, Mutex};

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

    // Use Arc<Mutex> to allow shared access to videoconvert
    let videoconvert = Arc::new(Mutex::new(videoconvert));

    // Add elements to the pipeline (ensure videoconvert is added)
    pipeline
        .add_many(&[
            &rtspsrc,
            &decodebin,
            &videoconvert.lock().unwrap(),
            &autovideosink,
        ])
        .expect("Failed to add elements to the pipeline");

    // Connect to rtspsrc's pad-added signal dynamically
    rtspsrc.connect_pad_added({
        let decodebin = decodebin.clone();
        move |_, pad| {
            // When a pad is added by rtspsrc, link it to decodebin
            let decodebin_pad = decodebin.static_pad("sink").unwrap();
            pad.link(&decodebin_pad)
                .expect("Failed to link rtspsrc to decodebin");
        }
    });

    // Handle the decodebin's dynamic pads to connect them to videoconvert
    decodebin.connect_pad_added({
        let videoconvert = Arc::clone(&videoconvert);
        move |_, pad| {
            // When decodebin emits a pad, link it to videoconvert
            if let Some(sinkpad) = videoconvert.lock().unwrap().static_pad("sink") {
                pad.link(&sinkpad)
                    .expect("Failed to link decodebin to videoconvert");
            }
        }
    });

    // Link videoconvert to autovideosink after adding videoconvert to the pipeline
    videoconvert
        .lock()
        .unwrap()
        .link(&autovideosink)
        .expect("Failed to link videoconvert to autovideosink");

    // Start playing the pipeline
    pipeline
        .set_state(State::Playing)
        .expect("Failed to set pipeline to Playing");

    // Wait until an error or EOS (End Of Stream)
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => {
                println!("End of Stream");
                break;
            }
            MessageView::Error(err) => {
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

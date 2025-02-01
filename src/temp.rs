use gstreamer::prelude::*;
use gstreamer_app::AppSink; // Correct import from gstreamer_app
use opencv::core::MatTraitConst;
use opencv::highgui; // For imshow and waitKey
use opencv::prelude::*; // Import the necessary OpenCV traits // For mat_size()

#[tokio::main]
async fn main() {
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
    // Correctly access appsink and pull frames
    let appsink = pipeline.by_name("appsink").unwrap();
    let appsink = appsink.dynamic_cast::<AppSink>().unwrap(); // Cast to AppSink
    let mut counter = 0;
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        counter += 1;
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
            _ => {
                match appsink.pull_sample() {
                    Ok(sample) => {
                        let buffer = sample.buffer().unwrap();
                        let map = buffer.map_readable().unwrap();
                        let raw_data = map.as_slice();

                        // Specify the correct type (e.g., u8) for from_bytes
                        let frame = opencv::core::Mat::from_bytes::<u8>(raw_data).unwrap();

                        // Correctly unwrap mat_size()
                        let frame_size = frame.mat_size();
                        println!("{}.Frame size: {:?}", counter, frame_size);

                        highgui::imshow("Video", &frame).unwrap();
                        if highgui::wait_key(1).unwrap() == 27 {
                            // ESC key to exit
                            break;
                        }
                    }
                    Err(_) => {
                        println!("No more frames to retrieve.");
                        break;
                    }
                }
            } // _ => (),
        }
    }

    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Failed to set pipeline to Null");
}

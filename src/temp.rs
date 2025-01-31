mod args;
mod msg;
mod node;

use crate::node::common::Message;
use clap::Parser;
#[allow(unused_imports)]
// use msg::proto::LidarData;
use msg::stream::LidarData;
use std::collections::VecDeque;
use std::f32::consts::PI;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let _guard = node::common::init_tracing_subscriber();

    let r = 1.0;
    let lidar_capacity = 150;
    let mut lidar_data_x_history = VecDeque::with_capacity(lidar_capacity);
    let mut lidar_data_y_history = VecDeque::with_capacity(lidar_capacity);

    // const N: usize = 30;
    const N: usize = 360;
    // for i in 0..N {
    let mut i = 0;
    loop {
        i += 1;

        // let (x, y) = (
        //     r * f32::sin(((i as f32) * 2.0 * PI) / ((N - 1) as f32)),
        //     r * f32::cos(((i as f32) * 2.0 * PI) / ((N - 1) as f32)),
        // );

        // let (y, x) = (
        //     r * f32::sin(((i as f32) * 2.0 * PI as f32) / (((N / 1) - 1) as f32)),
        //     r * f32::cos(((i as f32) * 2.0 * PI as f32) / (((N / 2) - 1) as f32)),
        // );

        let (x, y) = (
            r * f32::sin(((i as f32) * 2.0 * PI as f32) / (((N / 1) - 1) as f32)),
            r * f32::cos(((i as f32) * 2.0 * PI as f32) / (((N / 2) - 1) as f32)),
        );

        lidar_data_x_history.push_back(x);
        lidar_data_y_history.push_back(y);
        if lidar_data_x_history.len() > lidar_capacity {
            lidar_data_x_history.pop_front();
            lidar_data_y_history.pop_front();
        }

        println!(
            "Lidar Data: {:?}, {:?} with size: {:?}",
            lidar_data_x_history,
            lidar_data_y_history,
            lidar_data_y_history.len()
        );

        let ld = LidarData {
            home_x: x,
            home_y: y,
            lidar_data_x_history: lidar_data_x_history.iter().map(|x| *x).collect(),
            lidar_data_y_history: lidar_data_y_history.iter().map(|y| *y).collect(),
        };
        println!("Original LidarData: {:?}", ld);

        let ser = ld.ser();
        println!("Serializing LidarData: {:?}", ser);

        let deser = ld.deser(&ser);
        println!("Deserializing LidarData: {:?}", deser);

        let publisher = node::Publisher::new(
            "test_1",
            args.mode.as_str(),
            None,
            args.endpoints.iter().map(|x| x.as_str()).collect(),
        )
        .await
        .unwrap();

        publisher.publish(ld.clone()).await;

        // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        // tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        // tokio::time::sleep(std::time::Duration::from_micros(10)).await;
        // tokio::time::sleep(std::time::Duration::from_micros(1)).await;
        // tokio::time::sleep(std::time::Duration::from_nanos(10)).await;
        // tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
    }
}

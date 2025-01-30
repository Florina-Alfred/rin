mod msg;
mod node;

use crate::node::common::Message;
use msg::proto::LidarData;

#[tokio::main]
async fn main() {
    let mut data = Vec::new();

    for i in 0..3 {
        data.push((i / 100) as f32);
    }

    for _ in 0..5 {
        let _ = data.iter_mut().map(|x| *x = *x + 1.0).collect::<Vec<_>>();
        let ld = LidarData {
            home_x: 1,
            home_y: 2,
            lidar_data: data.clone(),
        };

        let ser = ld.ser();
        println!("Serializing LidarData: {:?}", ser);

        let deser = ld.deser(&ser);
        println!("Deserializing LidarData: {:?}", deser);

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

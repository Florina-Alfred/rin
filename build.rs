extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/msg/proto/SimpleMessage.proto"], &["src/"]).unwrap();
    prost_build::compile_protos(&["src/msg/proto/LidarData.proto"], &["src/"]).unwrap();
}

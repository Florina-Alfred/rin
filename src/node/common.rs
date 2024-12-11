use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio;
use tracing::info;

pub trait Message {
    async fn next(&mut self) -> Option<&mut Self>
    where
        Self: Sized;
    fn ser(&self) -> String
    where
        Self: Serialize,
    {
        let serialized = serde_json::to_string(&self).unwrap();
        return serialized;
    }
    fn deser(&self, msg: &String) -> Self
    where
        Self: for<'de> Deserialize<'de> + Debug,
    {
        let deserialized: Self = serde_json::from_str(&msg).unwrap();
        return deserialized;
    }
}

pub fn logger(message: String) {
    tokio::spawn(async move {
        // println!("{}", message);
        info!(message);
    });
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

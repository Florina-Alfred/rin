use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tokio;

pub trait Message {
    fn next(&mut self) -> Option<&mut Self>
    where
        Self: Sized;
    fn ser(&self) -> String
    where
        Self: Serialize,
    {
        let serialized = serde_json::to_string(&self).unwrap();
        return serialized;
    }
    fn deser(&self, msg: String) -> Self
    where
        Self: for<'de> Deserialize<'de> + Debug,
    {
        let deserialized: Self = serde_json::from_str(&msg).unwrap();
        return deserialized;
    }
}

pub fn logger(message: String) {
    tokio::spawn(async move {
        println!("{}", message);
    });
}

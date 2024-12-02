pub trait Message {
    fn update(&mut self, msg: String);
}

#[derive(Default, Debug, Clone)]
pub struct Stream {
    start: Option<u32>,
    num: u32,
}

impl Stream {
    #[allow(dead_code)]
    pub fn new(start: Option<u32>) -> Self {
        if let Some(start) = start {
            Stream {
                start: Some(start),
                num: start as u32,
            }
        } else {
            Stream {
                start: None,
                num: 0,
            }
        }
    }
}

impl Iterator for Stream {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.num >= self.start.unwrap() + 10 {
            return None;
        }
        self.num += 5;
        Some(self.num)
    }
}

impl Message for Stream {
    fn update(&mut self, msg: String) {
        self.num = msg.parse::<u32>().unwrap();
        println!("Stream update: {}", msg);
    }
}

#[derive(Default, Debug, Clone)]
struct MachineStruct {
    message: String,
    count: u32,
}

impl Message for MachineStruct {
    fn update(&mut self, msg: String) {
        self.message = msg;
        self.count += 1;
    }
}

#[derive(Debug, Default, Clone)]
struct UserStruct {
    old: String,
    new: String,
}

impl Message for UserStruct {
    fn update(&mut self, msg: String) {
        self.old = self.new.clone();
        self.new = msg.clone();
    }
}

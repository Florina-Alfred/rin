#[derive(Debug, Clone)]
pub struct Stream {
    start: Option<u32>,
    num: u32,
}
impl Stream {
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
        self.num += 2;
        Some(self.num)
    }
}

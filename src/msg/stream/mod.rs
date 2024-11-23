#[derive(Debug, Clone)]
pub struct Stream {
    a: i32,
}
impl Stream {
    pub fn new(start: Option<u32>) -> Self {
        if let Some(start) = start {
            Stream { a: start as i32 }
        } else {
            Stream { a: 0 }
        }
    }
}
impl Iterator for Stream {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.a >= 10_000 {
            return None;
        }
        self.a += 2;
        Some(self.a)
    }
}

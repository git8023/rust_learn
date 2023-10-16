#[derive(PartialEq, Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }

    pub fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

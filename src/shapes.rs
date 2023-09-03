pub struct Circle {
    radius: usize,
    color: u32,
}

impl Circle {
    pub fn new(radius: usize, color: u32) -> Self {
        return Circle { radius, color };
    }
}

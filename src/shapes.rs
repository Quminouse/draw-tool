pub struct Circle {
    pub radius: usize,
    pub color: u32,
}

impl Circle {
    pub fn new(radius: usize, color: u32) -> Self {
        return Circle { radius, color };
    }
}

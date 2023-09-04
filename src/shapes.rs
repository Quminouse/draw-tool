use crate::canvas::{Canvas, Color};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Circle {
    pub radius: usize,
    pub color: Color,
}

#[wasm_bindgen]
impl Circle {
    pub fn new(radius: usize, color: Color) -> Self {
        return Circle { radius, color };
    }
}

#[wasm_bindgen]
impl Canvas {
    pub fn draw_circle(&mut self, x: usize, y: usize, circle: Circle) {
        for index in 0..self.size() {
            let p2 = self.get_coordinate(index).unwrap();
            if x.abs_diff(p2.x).pow(2) + y.abs_diff(p2.y).pow(2) < circle.radius.pow(2) {
                *self.get_pixel_mut(p2).unwrap() = circle.color.as_u32();
            }
        }
    }
}

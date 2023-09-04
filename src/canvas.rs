use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
pub enum CanvasError {
    IndexError,
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
}
impl Pixel {
    fn new(x: usize, y: usize) -> Self {
        return Pixel { x, y };
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<u32>,
}
impl Canvas {
    pub fn size(&self) -> usize {
        return self.width * self.height;
    }

    pub fn get_index(&self, pixel: Pixel) -> Result<usize, CanvasError> {
        let index = pixel.y * self.width + pixel.x;
        if index <= self.pixels.len() {
            return Ok(index);
        }
        Err(CanvasError::IndexError)
    }

    pub fn get_pixel(&self, pixel: Pixel) -> Result<&u32, CanvasError> {
        let index = self.get_index(pixel)?;
        Ok(self.pixels.get(index).unwrap())
    }
    pub fn get_pixel_mut(&mut self, pixel: Pixel) -> Result<&mut u32, CanvasError> {
        let index = self.get_index(pixel)?;
        Ok(self.pixels.get_mut(index).unwrap())
    }
    pub fn get_coordinate(&self, index: usize) -> Result<Pixel, CanvasError> {
        if index <= self.pixels.len() {
            let x = index % self.width;
            let y = (index - x) / self.width;
            return Ok(Pixel::new(x, y));
        }
        Err(CanvasError::IndexError)
    }
    pub fn set_pixel(&mut self, pixel: Pixel, color: Color) -> Result<(), CanvasError> {
        *self.get_pixel_mut(pixel)? = color.as_u32();
        Ok(())
    }
}
#[wasm_bindgen]
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        return Canvas {
            width,
            height,
            pixels: vec![0; width * height],
        };
    }
    pub fn get_pixels_ptr(&self) -> *const u32 {
        return self.pixels.as_ptr();
    }
    pub fn fill(&mut self, color: Color) {
        self.pixels.fill(color.as_u32());
    }
    pub fn draw_circle(&mut self, x: usize, y: usize, radius: usize, color: Color) {
        for index in 0..self.size() {
            let p2 = self.get_coordinate(index).unwrap();
            if x.abs_diff(p2.x).pow(2) + y.abs_diff(p2.y).pow(2) <= radius.pow(2) {
                *self.get_pixel_mut(p2).unwrap() = color.as_u32();
            }
        }
    }
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        todo!();
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[wasm_bindgen]
impl Color {
    pub fn new(b: u8, g: u8, r: u8) -> Self {
        return Color { r, g, b, a: 255 };
    }
    pub fn as_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32) | ((self.a as u32) << 24)
    }
}


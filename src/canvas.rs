use crate::shapes::Circle;

#[derive(Debug)]
pub enum CanvasError {
    IndexError,
}

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        return Canvas {
            width,
            height,
            pixels: vec![0; width * height],
        };
    }
    pub fn fill(&mut self, value: u32) {
        self.pixels.fill(value);
    }
    pub fn get_index(&self, x: usize, y: usize) -> Result<usize, CanvasError> {
        let index = y * self.width + x;
        if index <= self.pixels.len() {
            return Ok(index);
        }
        Err(CanvasError::IndexError)
    }
    pub fn get_pixels(&self) -> &Vec<u32> {
        return &self.pixels;
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<&u32, CanvasError> {
        let index = self.get_index(x, y)?;
        Ok(self.pixels.get(index).unwrap())
    }
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Result<&mut u32, CanvasError> {
        let index = self.get_index(x, y)?;
        Ok(self.pixels.get_mut(index).unwrap())
    }
    pub fn get_coordinate(&self, index: usize) -> Result<(usize, usize), CanvasError> {
        if index <= self.pixels.len() {
            let x = index % self.width;
            let y = (index - x) / self.width;
            return Ok((x, y));
        }
        Err(CanvasError::IndexError)
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), CanvasError> {
        *self.get_pixel_mut(x, y)? = color;
        Ok(())
    }
    pub fn draw_circle(&mut self, x: usize, y: usize, circle: Circle) {
        for (index, pixel) in self.pixels.iter().enumerate() {
            let (x2, y2) = self.get_coordinate(index).unwrap();
            if x * x + y * y <= circle.radius * circle.radius {
                println!("x: {}, y: {}", x, y);
            }
        }
    }
}

impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.pixels.chunks(self.width) {
            for pixel in chunk.iter() {
                write!(f, "{:#01x} ", pixel)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

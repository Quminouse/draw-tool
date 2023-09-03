use draw_tool::canvas::Canvas;
use draw_tool::shapes::*;

fn main() {
    let mut canvas = Canvas::new(50, 40);
    canvas.fill(0xFF00FFFF);
    canvas.set_pixel(1, 1, 0xFF000000).unwrap();
    canvas.draw_circle(25, 20, Circle::new(10, 0xFFFFFFFF));
}

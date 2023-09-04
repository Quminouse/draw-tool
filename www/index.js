import { Canvas, Shape, Color } from "draw-tool";
import { memory } from "draw-tool/draw_tool_bg";

const js_canvas = document.getElementById("draw-canvas");
js_canvas.width = 800;
js_canvas.height = 600;

const ctx = js_canvas.getContext("2d");
const imageData = ctx.getImageData(0, 0, js_canvas.width, js_canvas.height);

const rust_canvas = Canvas.new(js_canvas.width, js_canvas.height);

rust_canvas.fill(Color.new(0, 0, 0));
render();

function render() {
    const u8array = new Uint8ClampedArray(memory.buffer, rust_canvas.get_pixels_ptr(), js_canvas.width * js_canvas.height * 4);
    imageData.data.set(u8array);
    ctx.putImageData(imageData, 0, 0);
}

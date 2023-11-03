use minifb::{Key, Window, WindowOptions};
use wavefront::Obj;

pub fn draw_line(
    buffer: &mut Vec<u32>,
    window: &Window,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
    color: u32,
) {
    let mut x0 = x0;
    let mut x1 = x1;
    let mut y0 = y0;
    let mut y1 = y1;
    let mut steep = false;
    if x0.abs_diff(x1) < y0.abs_diff(y1) {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    if x1 < x0 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    for x in x0..x1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = y0 as f32 * (1. - t) as f32 + y1 as f32 * t;
        let y = y as u32;
        if steep {
            set_pixel(window, buffer, y, x, color);
        } else {
            set_pixel(window, buffer, x, y, color);
        }
    }
}

pub fn set_pixel(window: &Window, buffer: &mut Vec<u32>, x: u32, y: u32, color: u32) {
    let width = window.get_size().0;
    let index = (y * width as u32 + x) as usize;
    if index < buffer.len() {
        buffer[index] = color;
    }
}

pub fn draw_wireframe(window: &Window, buffer: &mut Vec<u32>, model: Obj, color: u32) {
    let (width, height) = window.get_size();
    for [a, b, c] in model.triangles() {
        let x_a = ((a.position()[0] + 1.) * width as f32 / 2.) as u32;
        let y_a = ((a.position()[1] + 1.) * height as f32 / 2.) as u32;
        let x_b = ((b.position()[0] + 1.) * width as f32 / 2.) as u32;
        let y_b = ((b.position()[1] + 1.) * height as f32 / 2.) as u32;
        let x_c = ((c.position()[0] + 1.) * width as f32 / 2.) as u32;
        let y_c = ((c.position()[1] + 1.) * height as f32 / 2.) as u32;
        draw_line(buffer, &window, x_a, y_a, x_b, y_b, color);
        draw_line(buffer, &window, x_b, y_b, x_c, y_c, color);
        draw_line(buffer, &window, x_a, y_a, x_c, y_c, color);
    }
}

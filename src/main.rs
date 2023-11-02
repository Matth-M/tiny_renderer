use minifb::{Key, Window, WindowOptions};
mod colors;
mod drawing;
use crate::colors::{BLUE, GREEN, RED};
use crate::drawing::{draw_line, set_pixel};

fn main() {
    // Define the window dimensions and options
    let width = 800;
    let height = 600;
    let mut buffer: Vec<u32> = vec![0; width * height]; // A buffer to hold pixel data

    let mut window = Window::new("Pixel Renderer", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let model = wavefront::Obj::from_file("assets/diablo3_pose.obj").unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the window
        window.update_with_buffer(&buffer, width, height).unwrap();
        // In your rendering loop:
        // draw_line(&mut buffer, &window, 50, 200, 600, 400, RED);
        // draw_line(&mut buffer, &window, 13, 20, 80, 40, GREEN);
        // draw_line(&mut buffer, &window, 80, 40, 14, 20, BLUE);
        for [a, b, c] in model.triangles() {
            let x_a = ((a.position()[0] + 1.) * width as f32 / 2.) as u32;
            let y_a = ((a.position()[1] + 1.) * height as f32 / 2.) as u32;
            let x_b = ((b.position()[0] + 1.) * width as f32 / 2.) as u32;
            let y_b = ((b.position()[1] + 1.) * height as f32 / 2.) as u32;
            let x_c = ((c.position()[0] + 1.) * width as f32 / 2.) as u32;
            let y_c = ((c.position()[1] + 1.) * height as f32 / 2.) as u32;
            draw_line(&mut buffer, &window, x_a, y_a, x_b, y_b, GREEN);
            draw_line(&mut buffer, &window, x_b, y_b, x_c, y_c, GREEN);
            draw_line(&mut buffer, &window, x_a, y_a, x_c, y_c, GREEN);
        }
    }
}

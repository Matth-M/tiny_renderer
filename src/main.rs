use minifb::{Key, Window, WindowOptions};
mod colors;
mod drawing;
use crate::colors::Color;
use crate::drawing::draw_wireframe;

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
        draw_wireframe(&window, &mut buffer, model.clone(), Color::White);
    }
}

use drawing::triangle;
use minifb::{Key, Window, WindowOptions};
mod colors;
mod drawing;
use crate::colors::Color;
use crate::drawing::{draw_wireframe, triangle_line_sweep, Position};

fn main() {
    // Define the window dimensions and options
    let width = 800;
    let height = 600;
    let mut buffer: Vec<u32> = vec![0; width * height]; // A buffer to hold pixel data

    let mut window = Window::new("Pixel Renderer", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let _model = wavefront::Obj::from_file("assets/diablo3_pose.obj").unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the window
        window.update_with_buffer(&buffer, width, height).unwrap();
        let a = Position { x: 100, y: 100 };
        let b = Position { x: 300, y: 200 };
        let c = Position { x: 200, y: 400 };
        triangle(&mut buffer, &window, a, b, c, Color::Green);
        // draw_wireframe(&window, &mut buffer, model.clone(), Color::White);
    }
}

use minifb::{Key, Window, WindowOptions};
mod colors;
mod drawing;
use crate::colors::Color;
#[allow(unused_imports)]
use crate::drawing::{fill_triangle, render_model, ScreenPosition};

fn main() {
    // Define the window dimensions and options
    let width = 800;
    let height = 600;
    let mut buffer: Vec<u32> = vec![0; width * height]; // A buffer to hold pixel data

    let mut window = Window::new("Pixel Renderer", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    #[allow(unused_variables)]
    let head = wavefront::Obj::from_file("assets/head.obj").unwrap();
    let diablo3_pose = wavefront::Obj::from_file("assets/diablo3_pose.obj").unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the window
        drawing::render_model(&window, &mut buffer, &diablo3_pose);
        window.update_with_buffer(&buffer, width, height).unwrap();
        let a = ScreenPosition { x: 100, y: 100 };
        let b = ScreenPosition { x: 300, y: 200 };
        let c = ScreenPosition { x: 200, y: 400 };
        fill_triangle(
            &mut buffer,
            &window,
            &a,
            &b,
            &c,
            Color::from_u8_rgb(128, 0, 0),
        );
    }
}

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Your rendering code goes here
        // For example, you can modify the `buffer` to change pixel reds

        // Update the window
        window.update_with_buffer(&buffer, width, height).unwrap();
        // In your rendering loop:
        let x = 400; // Example X-coordinate
        let y = 300; // Example Y-coordinate

        // Set the pixel color in the buffer
        set_pixel(&window, &mut buffer, x, y, RED);
        draw_line(&mut buffer, &window, 50, 200, 600, 400, RED);
        draw_line(&mut buffer, &window, 13, 20, 80, 40, GREEN);
        draw_line(&mut buffer, &window, 80, 40, 14, 20, BLUE);
    }
}

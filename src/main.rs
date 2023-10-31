use minifb::{Key, Window, WindowOptions};

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
        // For example, you can modify the `buffer` to change pixel colors

        // Update the window
        window.update_with_buffer(&buffer, width, height).unwrap();
        // In your rendering loop:
        let x = 400; // Example X-coordinate
        let y = 300; // Example Y-coordinate
        let color = 0xFF_FF_00_FF; // Example color in ARGB format

        // Set the pixel color in the buffer
        buffer[y * width + x] = color;
        let (x0, y0, x1, y1) = (100, 200, 500, 400);
        draw_line(
            &mut buffer,
            width.try_into().unwrap(),
            x0,
            y0,
            x1,
            y1,
            color,
        );
    }
}

fn draw_line(buffer: &mut Vec<u32>, width: u32, x0: u32, y0: u32, x1: u32, y1: u32, color: u32) {
    let nb_steps = 1000;
    for t in (0..=nb_steps).map(|t| t as f32 / nb_steps as f32) {
        let x = (x0 as f32 + (x1 as f32 - x0 as f32) * t).round() as u32;
        let y = (y0 as f32 + (y1 as f32 - y0 as f32) * t).round() as u32;
        buffer[(y * width + x) as usize] = color;
    }
}

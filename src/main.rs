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
        // For example, you can modify the `buffer` to change pixel reds

        // Update the window
        window.update_with_buffer(&buffer, width, height).unwrap();
        // In your rendering loop:
        let x = 400; // Example X-coordinate
        let y = 300; // Example Y-coordinate
        let red = from_u8_rgb(255, 0, 0); // Example red in ARGB format
        let green = from_u8_rgb(0, 255, 0);
        let blue = from_u8_rgb(0, 100, 205);

        // Set the pixel color in the buffer
        set_pixel(&window, &mut buffer, x, y, red);
        draw_line(&mut buffer, &window, 50, 200, 600, 400, red);
        draw_line(&mut buffer, &window, 13, 20, 80, 40, green);
        draw_line(&mut buffer, &window, 80, 40, 14, 20, blue);
    }
}

fn draw_line(
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

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn set_pixel(window: &Window, buffer: &mut Vec<u32>, x: u32, y: u32, color: u32) {
    let width = window.get_size().0;
    buffer[(y * width as u32 + x) as usize] = color;
}

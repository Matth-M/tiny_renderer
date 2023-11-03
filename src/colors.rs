#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
}

impl Color {
    pub fn as_u32(&self) -> u32 {
        match self {
            Color::Red => from_u8_rgb(255, 0, 0),
            Color::Green => from_u8_rgb(0, 255, 0),
            Color::Blue => from_u8_rgb(0, 100, 205),
            Color::White => from_u8_rgb(255, 255, 255),
        }
    }
}
const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

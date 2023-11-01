pub const RED: u32 = from_u8_rgb(255, 0, 0); // Example red in ARGB format
pub const GREEN: u32 = from_u8_rgb(0, 255, 0);
pub const BLUE: u32 = from_u8_rgb(0, 100, 205);

const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

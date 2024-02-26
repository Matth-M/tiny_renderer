#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn as_u32(&self) -> u32 {
        return (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32;
    }

    pub fn red() -> Color {
        return Color { r: 255, g: 0, b: 0 };
    }

    pub fn green() -> Color {
        return Color { r: 0, g: 255, b: 0 };
    }

    pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> Color {
        return Color { r, g, b };
    }
}

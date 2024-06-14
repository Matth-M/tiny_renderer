use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(dead_code)]
impl Color {
    pub fn as_u32(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }

    pub fn red() -> Color {
        Color { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> Color {
        Color { r: 0, g: 255, b: 0 }
    }

    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "red: {}, green{}, blue:{}", self.r, self.g, self.b)
    }
}

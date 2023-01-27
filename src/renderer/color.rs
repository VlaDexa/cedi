#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for sdl2::pixels::Color {
    fn from(color: Color) -> Self {
        Self::RGBA(color.r, color.g, color.b, color.a)
    }
}

impl From<sdl2::pixels::Color> for Color {
    fn from(color: sdl2::pixels::Color) -> Self {
        Self::new(color.r, color.g, color.b, color.a)
    }
}

use sdl2::ttf::FontError;

use crate::renderer::color::Color;

use super::font_lib::FontLib;

pub struct Font<'ttf, 'file> {
    font: sdl2::ttf::Font<'ttf, 'file>,
}

#[derive(Debug)]
#[ace_it]
pub enum Error {
    Load(String),
    Render(FontError)
}

impl<'ttf, 'file> Font<'ttf, 'file> {
    pub fn new(lib: &'ttf FontLib, path: impl AsRef<std::path::Path>, point: u16) -> Result<Self, Error> {
        let font = lib.load_font(path, point)?;
        Ok(font)
    }

    pub fn render(&self, text: &str, color: Color) -> Result<sdl2::surface::Surface, Error> {
        self.font.render(text).blended(color).map_err(Error::Render)
    }
}

impl<'ttf, 'file> From<sdl2::ttf::Font<'ttf, 'file>> for Font<'ttf, 'file> {
    fn from(font: sdl2::ttf::Font<'ttf, 'file>) -> Self {
        Self { font }
    }
}
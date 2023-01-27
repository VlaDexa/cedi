use sdl2::ttf::FontError;

use crate::renderer::color::Color;

pub struct Font<'ttf, 'file> {
    font: sdl2::ttf::Font<'ttf, 'file>,
}

#[derive(Debug)]
#[ace_it]
pub enum Error {
    Load(String),
    Render(FontError),
}

impl<'ttf, 'file> Font<'ttf, 'file> {
    pub fn new(font: sdl2::ttf::Font<'ttf, 'file>) -> Self {
        Self { font }
    }

    pub fn render(&self, text: &str, color: Color) -> Result<sdl2::surface::Surface, Error> {
        self.font.render(text).blended(color).map_err(Error::Render)
    }
}

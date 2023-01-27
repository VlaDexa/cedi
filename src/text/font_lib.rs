use super::font::Font;

pub struct FontLib {
    lib: sdl2::ttf::Sdl2TtfContext,
}

#[derive(Debug)]
#[ace_it]
pub enum Error {
    Init(sdl2::ttf::InitError),
    Load(super::font::Error),
}

impl FontLib {
    pub fn new() -> Result<Self, Error> {
        let lib = sdl2::ttf::init()?;
        Ok(Self { lib })
    }

    pub fn load_font<'ttf, 'file>(
        &'ttf self,
        path: impl AsRef<std::path::Path>,
        point: u16,
    ) -> Result<Font<'ttf, 'file>, super::font::Error> {
        let font = self.lib.load_font(path, point)?;
        Ok(Font::new(font))
    }
}

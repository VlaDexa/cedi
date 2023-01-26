pub mod color;
pub mod shapes;

use std::fmt::Debug;

use sdl2::{
    render::{Canvas, TextureValueError, TextureCreator},
    video::{Window, WindowBuildError, WindowContext},
    IntegerOrSdlError, Sdl, VideoSubsystem,
};

use crate::text::font::{Font, Error};

use self::{shapes::rect::Rect, color::Color};

pub struct Renderer {
    _video: VideoSubsystem,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>
}

#[derive(Debug)]
#[ace_it]
pub enum CreationError {
    VideoInit(String),
    WindowCreation(WindowBuildError),
    CanvasCreation(IntegerOrSdlError),
}

#[derive(Debug)]
#[ace_it]
pub enum RenderError {
    Drawing(String),
    Rendering(Error),
    Placing(TextureValueError)
}

impl Renderer {
    pub fn new(context: &Sdl, title: &str, width: u32, height: u32) -> Result<Self, CreationError> {
        let video = context.video().map_err(CreationError::VideoInit)?;
        let window = video
            .window(title, width, height)
            .position_centered()
            .vulkan()
            .build()?;
        let canvas = window.into_canvas().build()?;
        let texture_creator = canvas.texture_creator();
        Ok(Self {
            _video: video,
            canvas,
            texture_creator
        })
    }

    pub fn place_rect(&mut self, rect: Rect, color: impl Into<Color>) -> Result<(), RenderError> {
        self.canvas.set_draw_color(color.into());
        self.canvas.draw_rect(rect.into())?;
        Ok(())
    }

    pub fn render_text(&mut self, text: &str, font: &Font, color: impl Into<Color>, location: Rect) -> Result<(), RenderError> {
        let surface = font.render(text, color.into())?;
        let texture = self.texture_creator.create_texture_from_surface(&surface)?;
        self.canvas.copy(&texture, None, Into::<sdl2::rect::Rect>::into(location))?;
        Ok(())
    }

    pub fn blit(&mut self) {
        self.canvas.present();
    }
}

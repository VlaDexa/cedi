pub mod color;
pub mod shapes;

use std::fmt::Debug;

use sdl2::{
    pixels::Color,
    render::Canvas,
    video::{Window, WindowBuildError},
    IntegerOrSdlError, Sdl, VideoSubsystem,
};

use self::shapes::rect::Rect;

pub struct Renderer {
    _video: VideoSubsystem,
    canvas: Canvas<Window>,
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
        Ok(Self { _video: video, canvas })
    }

    pub fn place_rect(&mut self, rect: Rect, color: impl Into<Color>) -> Result<(), RenderError> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(rect.into())?;
        Ok(())
    }

    pub fn blit(&mut self) {
        self.canvas.present();
    }
}

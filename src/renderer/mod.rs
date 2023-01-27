pub mod color;
pub mod shapes;

use std::fmt::Debug;

use sdl2::{
    render::{Canvas, TextureCreator, TextureValueError},
    video::{Window, WindowBuildError, WindowContext},
    IntegerOrSdlError, Sdl, VideoSubsystem,
};

use crate::text::font::{Error, Font};

use self::{color::Color, shapes::rect::Rect};

pub struct Renderer {
    video: VideoSubsystem,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    needs_refresh: bool,
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
    Placing(TextureValueError),
}

#[derive(Debug)]
#[ace_it]
pub enum RefreshError {
    RefreshRate(String),
}

impl Renderer {
    pub fn new(context: &Sdl, title: &str, width: u32, height: u32) -> Result<Self, CreationError> {
        let video = context.video().map_err(CreationError::VideoInit)?;
        let window = video
            .window(title, width, height)
            .position_centered()
            .vulkan()
            .build()?;
        let canvas = window.into_canvas().present_vsync().accelerated().build()?;
        let texture_creator = canvas.texture_creator();
        Ok(Self {
            video,
            canvas,
            texture_creator,
            needs_refresh: false,
        })
    }

    pub fn place_rect(&mut self, rect: Rect, color: impl Into<Color>) -> Result<(), RenderError> {
        self.canvas.set_draw_color(color.into());
        self.canvas.draw_rect(rect.into())?;
        self.needs_refresh = true;
        Ok(())
    }

    fn render_line<T: Into<Color> + Copy>(
        &mut self,
        text: &str,
        font: &Font,
        color: T,
        location: Rect,
    ) -> Result<(), RenderError> {
        assert!(!text.is_empty());
        assert!(!text.ends_with('\n'));
        let surface = font.render(text, color.into())?;
        let texture = self.texture_creator.create_texture_from_surface(&surface)?;
        self.canvas
            .copy(&texture, None, Into::<sdl2::rect::Rect>::into(location))?;
        self.needs_refresh = true;
        Ok(())
    }

    pub fn render_text<T: Into<Color> + Copy>(
        &mut self,
        text: impl AsRef<str>,
        font: &Font,
        color: T,
        location: Rect,
    ) -> Result<(), RenderError> {
        if text
            .as_ref()
            .strip_suffix('\n')
            .unwrap_or_else(|| text.as_ref())
            .is_empty()
        {
            return Ok(()); // Don't render empty text
        }
        let mut text = text.as_ref().replace('\t', "    ");
        let did_end = text.ends_with('\n');
        if !did_end {
            text.push('\n');
        }
        assert!(text.ends_with('\n'));
        let mut newlines_amount: u32 = u32::from(did_end);
        for char in text.as_bytes() {
            if *char == b'\n' {
                newlines_amount += 1;
            }
        }
        let rects = location.split_horizontal(newlines_amount);
        let lines = text.lines();
        for (mut line, rect) in lines.zip(rects) {
            if line.is_empty() {
                line = " ";
            }
            self.render_line(line, font, color, rect)?;
        }

        Ok(())
    }

    pub fn blit(&mut self) {
        if self.needs_refresh {
            self.canvas.present();
            self.needs_refresh = false;
        }
    }

    pub fn refresh_rate(&self) -> Result<u16, RefreshError> {
        self.video
            .current_display_mode(0)
            .map(|mode| turn_positive(mode.refresh_rate))
            .map_err(RefreshError::RefreshRate)
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        self.needs_refresh = true;
    }

    pub fn size(&self) -> (u32, u32) {
        self.canvas.window().size()
    }
}

fn turn_positive(mut num: i32) -> u16 {
    if num.is_negative() {
        num *= -1;
    }
    std::convert::TryInto::<u16>::try_into(num).unwrap()
}

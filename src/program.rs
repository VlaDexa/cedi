use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
};

use sdl2::{EventPump, Sdl};

use crate::{
    renderer::{
        color::Color,
        shapes::{point::Point, rect::Rect},
        CreationError, Renderer,
    },
    side::Side,
    text::{
        editable::Text,
        font::Font,
        font_lib::{Error, FontLib},
    },
};

pub struct Program {
    _context: Sdl,
    renderer: RefCell<Renderer>,
    event_pump: RefCell<EventPump>,
    font: FontLib,
    side: Side,
    shift_pressed: bool,
}

#[derive(Debug)]
#[ace_it]
pub enum InitError {
    ContextInit(String),
    Renderer(CreationError),
    EventPolling(EventError),
    FontInit(Error),
    FonTload(crate::text::font::Error),
}

#[derive(Debug)]
#[ace_it]
pub enum EventError {
    EventPolling(String),
}

impl Program {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, InitError> {
        let context = sdl2::init().map_err(InitError::ContextInit)?;
        let renderer = Renderer::new(&context, title, width, height)?;
        let event_pump = context
            .event_pump()
            .map_err(|err| InitError::EventPolling(err.into()))?;
        let font = FontLib::new()?;
        Ok(Self {
            _context: context,
            renderer: RefCell::new(renderer),
            event_pump: RefCell::new(event_pump),
            font,
            side: Side::new(Text::new()),
            shift_pressed: false,
        })
    }

    pub fn renderer(&self) -> RefMut<Renderer> {
        self.renderer.borrow_mut()
    }

    pub fn get_event(&self) -> Option<sdl2::event::Event> {
        self.event_pump.borrow_mut().poll_event()
    }

    pub fn process_event(&mut self) -> bool {
        while let Some(event) = self.get_event() {
            match event {
                sdl2::event::Event::Quit { .. } => return true,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Backspace),
                    ..
                } => {
                    self.side.as_mut().pop();
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::LShift),
                    ..
                } => {
                    self.shift_pressed = true;
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::LShift),
                    ..
                } => {
                    self.shift_pressed = false;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    self.side.as_mut().insert(keycode, self.shift_pressed);
                },
                sdl2::event::Event::MouseButtonDown { .. } => {
                    dbg!(&self.side);
                }
                _ => {}
            }
        }
        let font = self
            .load_font("./fonts/Roboto-Black.ttf", u16::MAX / 100)
            .unwrap();
        let mut render = self.renderer();
        let size = render.size();
        render.clear(Color::BLACK);
        if !self.side.as_ref().as_ref().is_empty() {
            render
                .render_text(
                    self.side.as_ref(),
                    &font,
                    Color::WHITE,
                    Rect::new(Point::new(0, 0), Point::new(size.0, size.1)),
                )
                .unwrap();
        }
        render.blit();
        false
    }

    pub fn load_file(&mut self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let file = std::fs::read_to_string(path)?;
        *self.side.as_mut() = Text::from(file);
        Ok(())
    }

    pub fn load_font(
        &self,
        path: impl AsRef<std::path::Path>,
        point: u16,
    ) -> Result<Font, crate::text::font::Error> {
        self.font.load_font(path, point)
    }
}

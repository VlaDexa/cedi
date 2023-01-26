use std::{fmt::Debug, cell::{RefCell, RefMut}};

use sdl2::{EventPump, Sdl};

use crate::{renderer::{CreationError, Renderer}, text::{font_lib::{FontLib, Error}, font::Font}};

pub struct Program {
    _context: Sdl,
    renderer: RefCell<Renderer>,
    event_pump: RefCell<EventPump>,
    font: FontLib
}

#[derive(Debug)]
#[ace_it]
pub enum InitError {
    ContextInit(String),
    Renderer(CreationError),
    EventPolling(EventError),
    FontInit(Error)
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
            font
        })
    }

    pub fn renderer(&self) -> RefMut<Renderer> {
        self.renderer.borrow_mut()
    }

    pub fn get_event(&self) -> Option<sdl2::event::Event> {
        self.event_pump.borrow_mut().poll_event()
    }

    pub fn load_font(&'_ self, path: impl AsRef<std::path::Path>, point: u16) -> Result<Font, crate::text::font::Error> {
        Font::new(&self.font, path, point)
    }
}

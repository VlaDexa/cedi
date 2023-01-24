use std::fmt::Debug;

use sdl2::{EventPump, Sdl};

use crate::renderer::{CreationError, Renderer};

pub struct Program {
    _context: Sdl,
    renderer: Renderer,
    event_pump: EventPump,
}

#[derive(Debug)]
#[ace_it]
pub enum InitError {
    ContextInit(String),
    Renderer(CreationError),
    EventPolling(EventError),
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
        Ok(Self {
            _context: context,
            renderer,
            event_pump,
        })
    }

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn get_event(&mut self) -> Option<sdl2::event::Event> {
        self.event_pump.poll_event()
    }
}

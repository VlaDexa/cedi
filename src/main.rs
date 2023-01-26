#![warn(clippy::pedantic, clippy::nursery)]

use program::Program;
use renderer::{
    color::Color,
    shapes::{point::Point, rect::Rect},
};

#[macro_use]
extern crate ace_it;

mod program;
mod renderer;
mod text;

const WINDOW_TITLE: &str = "CEDI";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let program = Program::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    let top_left = Point::new(100, 100);
    let down_right = Point::new(WINDOW_WIDTH - 100, WINDOW_HEIGHT - 100);
    let rect = Rect::new(top_left, down_right);
    let font = program.load_font("./fonts/Roboto-Black.ttf", 1000).unwrap();
    let mut render = program.renderer();
    render.render_text("Some text", &font, Color::WHITE, rect).unwrap();
    render.place_rect(rect, Color::RED).unwrap();
    render.blit();
    'main_loop: loop {
        while let Some(event) = program.get_event() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                _ => std::thread::sleep(std::time::Duration::from_millis(1000 / 60)),
            };
        }
    }
}

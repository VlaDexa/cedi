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

const WINDOW_TITLE: &str = "CEDI";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let mut program = Program::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    let top_left = Point::new((WINDOW_WIDTH / 8) as i32, (WINDOW_HEIGHT / 6) as i32);
    let down_right = Point::new(
        (WINDOW_WIDTH * 7 / 8) as i32,
        (WINDOW_HEIGHT * 5 / 6) as i32,
    );
    let rect = Rect::new(top_left, down_right);
    let render = program.renderer();
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

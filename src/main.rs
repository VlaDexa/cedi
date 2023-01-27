#![warn(clippy::pedantic, clippy::nursery)]

use program::Program;

#[macro_use]
extern crate ace_it;

mod program;
mod renderer;
mod side;
mod text;

const WINDOW_TITLE: &str = "CEDI";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
fn main() {
    let mut program = Program::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    program.load_file("./src/text/mod.rs").unwrap();
    while !program.process_event() {}
}

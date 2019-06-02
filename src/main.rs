mod enums;
mod eventhandler;
mod mainstate;
mod shapes;

use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use mainstate::MainState;
use rand::Rng;
use std::time::Instant;

pub fn main() -> GameResult  {
    let (mut ctx, mut event_loop) = ContextBuilder::new("rusty_tetris", "gcx11")
        .window_setup(ggez::conf::WindowSetup::default().title("Rusty Tetris"))
        .build()?;

    let mut state = MainState::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut state)
}
mod enums;
mod eventhandler;
mod mainstate;
mod shapes;

use ggez::{ContextBuilder, GameResult};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::event;
use mainstate::MainState;

pub fn main() -> GameResult  {
    let (mut ctx, mut event_loop) = ContextBuilder::new("rusty_tetris", "gcx11")
        .window_setup(WindowSetup::default().title("Rusty Tetris"))
        .window_mode(WindowMode::default().dimensions(350.0, 300.0))
        .build()?;

    let mut state = MainState::new(&mut ctx)?;

    event::run(&mut ctx, &mut event_loop, &mut state)
}
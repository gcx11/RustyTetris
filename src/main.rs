mod enums;
mod eventhandler;
mod game;
mod mainstate;
mod menu;
mod scene;
mod shapes;

use ggez::{ContextBuilder, GameResult};
use ggez::conf::{WindowSetup, WindowMode};
use ggez::event;
use game::Game;
use crate::menu::Menu;

pub fn main() -> GameResult  {
    let (mut ctx, mut event_loop) = ContextBuilder::new("rusty_tetris", "gcx11")
        .window_setup(WindowSetup::default().title("Rusty Tetris"))
        .window_mode(WindowMode::default().dimensions(350.0, 300.0))
        .build()?;

    let mut game = Game::new(Box::new(Menu::new(None)));
    event::run(&mut ctx, &mut event_loop, &mut game)
}
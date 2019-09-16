use ggez::Context;
use ggez::error::GameResult;
use crate::game::GameContext;

pub trait Scene {
    fn update(&mut self, _ctx: &mut Context, game_context: &mut GameContext) -> GameResult;

    fn draw(&mut self, _ctx: &mut Context, game_context: &mut GameContext) -> GameResult;
}
use ggez::event::EventHandler;
use ggez::Context;
use ggez::error::GameError;
use ggez::error::GameResult;

pub struct Game {
    pub scene: Box<EventHandler>
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.scene.update(_ctx)
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.scene.draw(_ctx)
    }
}
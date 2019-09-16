use ggez::event::EventHandler;
use ggez::Context;
use ggez::error::GameResult;
use crate::scene::Scene;

pub struct Game {
    scene: Box<Scene>,
    game_context: GameContext
}

pub struct GameContext {
    next_scene: Option<Box<Scene>>
}

impl GameContext {
    fn new() -> Self {
        return GameContext { next_scene: None }
    }

    pub fn change_scene(&mut self, next_scene: Box<Scene>) {
        self.next_scene = Some(next_scene);
    }

    fn retrieve_next_scene(&mut self) -> Option<Box<Scene>> {
        return self.next_scene.take();
    }
}

impl Game {
    pub fn new(event_handler: Box<Scene>) -> Self {
        return Game { scene: event_handler, game_context: GameContext::new() }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if let Some(next_scene) = self.game_context.retrieve_next_scene() {
            self.scene = next_scene;
        };

        self.scene.update(_ctx, &mut self.game_context)
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.scene.draw(_ctx, &mut self.game_context)
    }
}
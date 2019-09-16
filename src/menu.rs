use crate::scene::Scene;
use ggez::Context;
use crate::game::GameContext;
use ggez::error::GameResult;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::graphics;
use crate::mainstate::MainState;
use ggez::graphics::*;

pub struct Menu {
    last_score: Option<u64>
}

impl Menu {
    pub fn new(score: Option<u64>) -> Self {
        return Menu { last_score: score }
    }
}

impl Scene for Menu {
    fn update(&mut self, _ctx: &mut Context, game_context: &mut GameContext) -> GameResult<()>  {
        if keyboard::is_key_pressed(_ctx, KeyCode::Return) {
            game_context.change_scene(Box::new(MainState::new()?));
        }

       Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context, game_context: &mut GameContext) -> GameResult<()> {
        graphics::clear(_ctx, BLACK);

        let text = Text::new(
            TextFragment {
                text: "Press ENTER to start!".into(),
                color: None,
                font: None,
                scale: Some(Scale::uniform(24.0)),
            }
        );

        let param = DrawParam::default().dest([50.0, 150.0]);
        graphics::draw(_ctx, &text, param)?;

        if let Some(score) = self.last_score {
            let score_text = Text::new(
                TextFragment {
                    text: format!("Previous score: {}", score).into(),
                    color: None,
                    font: None,
                    scale: Some(Scale::uniform(24.0)),
                }
            );

            let param = DrawParam::default().dest([80.0, 200.0]);
            graphics::draw(_ctx, &score_text, param)?;
        }

        return graphics::present(_ctx);
    }
}
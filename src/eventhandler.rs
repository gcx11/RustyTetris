use crate::enums::*;
use crate::mainstate::MainState;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use std::time::Instant;
use crate::scene::Scene;
use crate::game::GameContext;
use crate::menu::Menu;

impl Scene for MainState {
    fn update(&mut self, _ctx: &mut Context, game_context: &mut GameContext) -> GameResult<()> {
        match self.game_state {
            GameState::Idle => {
                self.spawn_new_piece();

                if self.game_state == GameState::GameOver {
                    game_context.change_scene(Box::new(Menu::new(Some(self.score))))
                }
            }

            GameState::Deleting => {
                self.delete_full_rows();
            }

            GameState::Falling => {
                let current = Instant::now();

                if (current - self.last_move_gravity) > self.gravity_speed {
                    self.update_area(Direction::Down);
                    self.last_move_gravity = current;
                } else if (current - self.last_move_player).as_millis() > 200 {
                    if keyboard::is_key_pressed(_ctx, KeyCode::Down) || keyboard::is_key_pressed(_ctx, KeyCode::S) {
                        self.update_area(Direction::Down);
                        self.last_move_player = current;
                    } else if keyboard::is_key_pressed(_ctx, KeyCode::Left) || keyboard::is_key_pressed(_ctx, KeyCode::A) {
                        self.update_area(Direction::Left);
                        self.last_move_player = current;
                    } else if keyboard::is_key_pressed(_ctx, KeyCode::Right) || keyboard::is_key_pressed(_ctx, KeyCode::D) {
                        self.update_area(Direction::Right);
                        self.last_move_player = current;
                    } else if keyboard::is_key_pressed(_ctx, KeyCode::Up) || keyboard::is_key_pressed(_ctx, KeyCode::W) {
                        self.rotate();
                        self.last_move_player = current;
                    }
                }
            }

            GameState::GameOver => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, game_context: &mut GameContext) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        for (i, row) in self.game_area.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let color = block_type_to_color(*cell);

                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(j as f32 * 20.0, i as f32 * 20.0, 20.0, 20.0),
                    color
                )?;
                graphics::draw(ctx, &rectangle, DrawParam::default())?;
            }
        }

        // text
        let text = Text::new(
            TextFragment {
                text: format!("Score: {}", self.score.to_string()).into(),
                color: None,
                font: None,
                scale: Some(Scale::uniform(24.0)),
            }
        );

        let param = DrawParam::default().dest([220.0, 0.0]);
        graphics::draw(ctx, &text, param)?;

        // next shape
        for part in self.next_shape.get_parts() {
            let color = block_type_to_color(part.block_type);

            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(part.x as f32 * 20.0 + 250.0, part.y as f32 * 20.0 + 100.0, 20.0, 20.0),
                color
            )?;
            graphics::draw(ctx, &rectangle, DrawParam::default())?;
        }

        return graphics::present(ctx);
    }
}

fn block_type_to_color(block_type: u8) -> Color {
    match block_type {
        2 | 3 => Color::from_rgb(250, 20, 0),
        1 => Color::from_rgb(0, 50, 150),
        _ => Color::from_rgb(200, 200, 200)
    }
}
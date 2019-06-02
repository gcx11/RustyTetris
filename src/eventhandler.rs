use crate::enums::*;
use crate::mainstate::MainState;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use rand::Rng;
use std::time::Instant;

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        match self.game_state {
            GameState::Idle => {
                self.spawn_new_piece();
            }

            GameState::Deleting => {
                self.delete_full_rows();
            }

            GameState::Falling => {
                let current = Instant::now();

                if (current - self.last_move_gravity).as_secs() > 1 {
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
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, BLACK);

        for (i, row) in self.game_area.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let color = match cell {
                    2 => Color::from_rgb(255, 0, 0),
                    1 => Color::from_rgb(0, 0, 255),
                    _ => Color::from_rgb(0, 255, 0)
                };

                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(j as f32 * 20.0, i as f32 * 20.0, 20.0, 20.0),
                    color
                )?;
                graphics::draw(ctx, &rectangle, DrawParam::default())?;
            }
        }

        return graphics::present(ctx);
    }
}
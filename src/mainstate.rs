use crate::enums::*;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use rand::Rng;
use std::time::Instant;

const X_SIZE: usize = 10;
const Y_SIZE: usize = 15;

pub struct MainState {
    pub game_area: [[u8; X_SIZE]; Y_SIZE],
    pub game_state: GameState,
    pub last_move_gravity: Instant,
    pub last_move_player: Instant
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut s = MainState {
            game_area: [[0; X_SIZE]; Y_SIZE],
            game_state: GameState::Idle,
            last_move_gravity: Instant::now(),
            last_move_player: Instant::now()
        };

        Ok(s)
    }

    pub fn update_area(&mut self, direction: Direction) {
        match self.try_move(direction) {
            MoveResult::Freeze => self.freeze(),
            MoveResult::Allowed => self.move_piece(direction),
            MoveResult::Denied => {}
        }
    }

    fn try_move(&self, direction: Direction) -> MoveResult {
        for j in 0..Y_SIZE {
            for i in 0..X_SIZE {
                let current = self.game_area[j][i];

                let (new_i, new_j) = match direction {
                    Direction::Up => (i, j.overflowing_sub(1).0),
                    Direction::Down => (i, j + 1),
                    Direction::Left => (i.overflowing_sub(1).0, j),
                    Direction::Right => (i + 1, j),
                };

                let next= if (0..X_SIZE).contains(&new_i) && (0..Y_SIZE).contains(&new_j) {
                    self.game_area[new_j][new_i]
                } else {
                    1
                };

                if next == 1 && current > 1 {
                    if direction == Direction::Down {
                        return MoveResult::Freeze
                    } else {
                        return MoveResult::Denied
                    }
                }
            }
        }

        return MoveResult::Allowed;
    }

    fn freeze(&mut self) {
        for j in 0..Y_SIZE {
            for i in 0..X_SIZE {
                if self.game_area[j][i] > 1 {
                    self.game_area[j][i] = 1;
                }
            }
        }

        self.game_state = GameState::Deleting;
    }

    fn move_piece(&mut self, direction: Direction) {
        let mut clone = self.game_area.clone();

        for j in 0..Y_SIZE {
            for i in 0..X_SIZE {
                let (old_i, old_j) = match direction {
                    Direction::Up => (i, j + 1),
                    Direction::Down => (i, j.overflowing_sub(1).0),
                    Direction::Left => (i + 1, j),
                    Direction::Right => (i.overflowing_sub(1).0, j),
                };

                let old = self.game_area[j][i];

                let new = if (0..X_SIZE).contains(&old_i) && (0..Y_SIZE).contains(&old_j) {
                    self.game_area[old_j][old_i]
                } else {
                    0
                };

                if old > 1 && new == 0 || old == 0 && new > 1 {
                    clone[j][i] = new
                } else if old > 1 && new == 1 {
                    clone[j][i] = 0
                }
            }
        }

        self.game_area = clone;
    }

    pub fn delete_full_rows(&mut self) {
        let mut new = [[0; X_SIZE]; Y_SIZE];

        let mut row_number = Y_SIZE - 1;

        for row in (0..Y_SIZE).rev() {
            if !self.game_area[row].iter().all(|&x| x == 1) {
                new[row_number] = self.game_area[row].clone();

                if row_number > 0 {
                    row_number -= 1;
                }
            }
        }

        self.game_area = new;
        self.game_state = GameState::Idle;
    }

    pub fn spawn_new_piece(&mut self) {
        let new_shape: Shape = rand::random();

        match new_shape {
            Shape::Square => {
                self.game_area[0][0] = 2;
                self.game_area[0][1] = 2;
                self.game_area[1][0] = 2;
                self.game_area[1][1] = 2;
            }

            Shape::LOne => {
                self.game_area[0][0] = 2;
                self.game_area[0][1] = 2;
                self.game_area[0][2] = 2;
                self.game_area[1][2] = 2;
            }

            Shape::LTwo => {
                self.game_area[1][0] = 2;
                self.game_area[1][1] = 2;
                self.game_area[1][2] = 2;
                self.game_area[0][2] = 2;
            }

            Shape::LThree => {
                self.game_area[0][0] = 2;
                self.game_area[0][1] = 2;
                self.game_area[1][1] = 2;
                self.game_area[2][1] = 2;
            }

            Shape::LFour => {
                self.game_area[0][0] = 2;
                self.game_area[1][0] = 2;
                self.game_area[0][1] = 2;
                self.game_area[0][2] = 2;
            }

            Shape::RowVertical => {
                self.game_area[0][0] = 2;
                self.game_area[1][0] = 2;
                self.game_area[2][0] = 2;
                self.game_area[3][0] = 2;
            }

            Shape::RowHorizontal => {
                self.game_area[0][0] = 2;
                self.game_area[0][1] = 2;
                self.game_area[0][2] = 2;
                self.game_area[0][3] = 2;
            }

            Shape::ZigOne => {
                self.game_area[0][1] = 2;
                self.game_area[1][1] = 2;
                self.game_area[1][0] = 2;
                self.game_area[2][0] = 2;
            }

            Shape::ZigTwo => {
                self.game_area[0][0] = 2;
                self.game_area[0][1] = 2;
                self.game_area[1][1] = 2;
                self.game_area[1][2] = 2;
            }

            Shape::ZigThree => {
                self.game_area[0][1] = 2;
                self.game_area[1][1] = 2;
                self.game_area[1][0] = 2;
                self.game_area[2][0] = 2;
            }

            Shape::ZigFour => {
                self.game_area[0][0] = 2;
                self.game_area[0][1] = 2;
                self.game_area[1][1] = 2;
                self.game_area[1][2] = 2;
            }

            Shape::TriangleUp => {
                self.game_area[0][0] = 2;
                self.game_area[0][1] = 2;
                self.game_area[0][2] = 2;
                self.game_area[1][1] = 2;
            }

            Shape::TriangleDown => {
                self.game_area[0][1] = 2;
                self.game_area[1][0] = 2;
                self.game_area[1][1] = 2;
                self.game_area[1][2] = 2;
            }

            Shape::TriangleLeft => {
                self.game_area[0][1] = 2;
                self.game_area[1][0] = 2;
                self.game_area[1][1] = 2;
                self.game_area[2][1] = 2;
            }

            Shape::TriangleRight => {
                self.game_area[0][0] = 2;
                self.game_area[1][0] = 2;
                self.game_area[1][1] = 2;
                self.game_area[2][0] = 2;
            }
        }

        self.game_state = GameState::Falling
    }
}
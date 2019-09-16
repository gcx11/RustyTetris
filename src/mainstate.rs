use crate::enums::*;
use ggez::GameResult;
use std::time::Instant;
use std::time::Duration;

const X_SIZE: usize = 10;
const Y_SIZE: usize = 15;

pub struct MainState {
    pub game_area: [[u8; X_SIZE]; Y_SIZE],
    pub game_state: GameState,
    pub last_move_gravity: Instant,
    pub last_move_player: Instant,
    pub next_shape: Shape,
    pub gravity_speed: Duration,
    pub score: u64
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let s = MainState {
            game_area: [[0; X_SIZE]; Y_SIZE],
            game_state: GameState::Idle,
            last_move_gravity: Instant::now(),
            last_move_player: Instant::now(),
            next_shape: rand::random(),
            gravity_speed: Duration::from_secs(1),
            score: 0
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
                let (previous_i, previous_j) = match direction {
                    Direction::Up => (i, j + 1),
                    Direction::Down => (i, j.overflowing_sub(1).0),
                    Direction::Left => (i + 1, j),
                    Direction::Right => (i.overflowing_sub(1).0, j),
                };

                let old = self.game_area[j][i];

                let new = if (0..X_SIZE).contains(&previous_i) && (0..Y_SIZE).contains(&previous_j) {
                    self.game_area[previous_j][previous_i]
                } else {
                    0
                };

                if old == 1 && new == 0 {
                    clone[j][i] = old
                } else if new > 1 {
                    clone[j][i] = new
                } else if new == 0 && old > 1 {
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
        let mut deleted_rows = 0;

        for row in (0..Y_SIZE).rev() {
            if !self.game_area[row].iter().all(|&x| x == 1) {
                new[row_number] = self.game_area[row].clone();

                if row_number > 0 {
                    row_number -= 1;
                }
            } else {
                deleted_rows += 1;
            }
        }

        self.game_area = new;
        self.game_state = GameState::Idle;
        self.score += 100 * deleted_rows;
    }

    pub fn spawn_new_piece(&mut self) {
        let new_shape = self.next_shape.clone();
        self.next_shape = rand::random();

        if self.gravity_speed.as_millis() > 200 {
            self.gravity_speed = self.gravity_speed - Duration::from_millis(10);
        }

        let parts = new_shape.get_parts();

        self.game_state = GameState::Falling;
        for part in parts {
            if self.game_area[part.y][part.x] != 0 {
                self.game_state = GameState::GameOver;
            }

            self.game_area[part.y][part.x] = part.block_type;
        }
    }

    pub fn rotate(&mut self) {
        let mut center_x: Option<usize> = None;
        let mut center_y: Option<usize> = None;

        for j in 0..Y_SIZE {
            for i in 0..X_SIZE {
                if self.game_area[j][i] == 3 {
                    center_x = Some(i);
                    center_y = Some(j);
                }
            }
        }

        if center_x.is_none() && center_y.is_none() {
            return
        }

        let center_x = center_x.unwrap() as i32;
        let center_y = center_y.unwrap() as i32;

        let mut clone = self.game_area.clone();

        for j in 0..Y_SIZE {
            for i in 0..X_SIZE {
                if clone[j][i] == 2 {
                    clone[j][i] = 0
                }
            }
        }

        for j in 0..Y_SIZE {
            for i in 0..X_SIZE {
                if self.game_area[j][i] == 2 {
                    let inverted_y = Y_SIZE as i32 - j as i32 - 1;
                    let inverted_center_y = Y_SIZE as i32 - center_y - 1;

                    let new_x = (inverted_y - inverted_center_y) + center_x;
                    let new_y = Y_SIZE as i32 - (center_x - i as i32) - inverted_center_y - 1;

                    if (0..X_SIZE as i32).contains(&new_x) && (0..Y_SIZE as i32).contains(&new_y) {
                        if clone[new_y as usize][new_x as usize] == 1 {
                            return
                        }

                        clone[new_y as usize][new_x as usize] = 2;
                    } else {
                        return;
                    }
                }
            }
        }

        self.game_area = clone;
    }
}
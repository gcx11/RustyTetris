#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub enum GameState {
    Idle,
    Falling,
}

pub enum MoveResult {
    Allowed,
    Denied,
    Freeze,
}

pub enum Shape {
    Square,
    LOne,
    LTwo,
    LThree,
    LFour,
    ZigOne,
    ZigTwo,
    RowHorizontal,
    RowVertical
}
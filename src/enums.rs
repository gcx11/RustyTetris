use enum_length_derive::EnumVariantCount;
use num_derive::FromPrimitive;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq)]
pub enum GameState {
    Idle,
    Falling,
    Deleting,
    GameOver
}

pub enum MoveResult {
    Allowed,
    Denied,
    Freeze,
}

#[derive(EnumVariantCount)]
#[derive(FromPrimitive)]
#[derive(Copy, Clone)]
pub enum Shape {
    Square,
    LOne,
    LTwo,
    LThree,
    LFour,
    ZigOne,
    ZigTwo,
    ZigThree,
    ZigFour,
    RowHorizontal,
    RowVertical,
    TriangleUp,
    TriangleDown,
    TriangleLeft,
    TriangleRight,
}
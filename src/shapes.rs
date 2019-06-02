use crate::enums::Shape;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0, 9) {
            0 => Shape::Square,
            1 => Shape::LOne,
            2 => Shape::LTwo,
            3 => Shape::LThree,
            4 => Shape::LFour,
            5 => Shape::RowHorizontal,
            6 => Shape::RowVertical,
            7 => Shape::ZigOne,
            8 => Shape::ZigTwo,
            _ => panic!("Shape out of range!"),
        }
    }
}
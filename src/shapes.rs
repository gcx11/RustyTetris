use crate::enums::Shape;
use num::FromPrimitive;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        let shape_variant = rng.gen_range(0, Shape::variant_count());

        FromPrimitive::from_usize(shape_variant).unwrap()
    }
}

pub struct ShapePart {
    pub x: usize,
    pub y: usize,
    pub block_type: u8
}

impl Shape {
    pub fn get_parts(&self) -> Vec<ShapePart> {
        return match self {
            Shape::Square => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 0, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 1, block_type: 2 }
                ]
            }

            Shape::LOne => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 0, block_type: 2 },
                    ShapePart { x: 2, y: 0, block_type: 3 },
                    ShapePart { x: 2, y: 1, block_type: 2 }
                ]
            }

            Shape::LTwo => {
                vec![
                    ShapePart { x: 1, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 2, block_type: 3 },
                    ShapePart { x: 0, y: 2, block_type: 2 }
                ]
            }

            Shape::LThree => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 0, block_type: 3 },
                    ShapePart { x: 1, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 2, block_type: 2 }
                ]
            }

            Shape::LFour => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 3 },
                    ShapePart { x: 0, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 0, block_type: 2 },
                    ShapePart { x: 2, y: 0, block_type: 2 }
                ]
            }

            Shape::RowVertical => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 0, y: 1, block_type: 2 },
                    ShapePart { x: 0, y: 2, block_type: 3 },
                    ShapePart { x: 0, y: 3, block_type: 2 }
                ]
            }

            Shape::RowHorizontal => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 0, block_type: 3 },
                    ShapePart { x: 2, y: 0, block_type: 2 },
                    ShapePart { x: 3, y: 0, block_type: 2 }
                ]
            }

            Shape::ZigOne => {
                vec![
                    ShapePart { x: 1, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 1, block_type: 3 },
                    ShapePart { x: 0, y: 1, block_type: 2 },
                    ShapePart { x: 0, y: 2, block_type: 2 }
                ]
            }

            Shape::ZigTwo => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 0, block_type: 3 },
                    ShapePart { x: 1, y: 1, block_type: 2 },
                    ShapePart { x: 2, y: 1, block_type: 2 }
                ]
            }

            Shape::ZigThree => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 0, y: 1, block_type: 3 },
                    ShapePart { x: 1, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 2, block_type: 2 }
                ]
            }

            Shape::ZigFour => {
                vec![
                    ShapePart { x: 0, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 0, block_type: 3 },
                    ShapePart { x: 2, y: 0, block_type: 2 }
                ]
            }

            Shape::TriangleUp => {
                vec![
                    ShapePart { x: 1, y: 0, block_type: 2 },
                    ShapePart { x: 0, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 1, block_type: 3 },
                    ShapePart { x: 2, y: 1, block_type: 2 }
                ]
            }

            Shape::TriangleDown => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 0, block_type: 3 },
                    ShapePart { x: 2, y: 0, block_type: 2 },
                    ShapePart { x: 1, y: 1, block_type: 2 }
                ]
            }

            Shape::TriangleLeft => {
                vec![
                    ShapePart { x: 1, y: 0, block_type: 2 },
                    ShapePart { x: 0, y: 1, block_type: 2 },
                    ShapePart { x: 1, y: 1, block_type: 3 },
                    ShapePart { x: 1, y: 2, block_type: 2 }
                ]
            }

            Shape::TriangleRight => {
                vec![
                    ShapePart { x: 0, y: 0, block_type: 2 },
                    ShapePart { x: 0, y: 1, block_type: 3 },
                    ShapePart { x: 1, y: 1, block_type: 2 },
                    ShapePart { x: 0, y: 2, block_type: 2 }
                ]
            }
        }
    }
}
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
pub mod arithmetic; pub mod geometry;

use rand::Rng;
use rand::rngs::SmallRng;
use crate::models::Question;

pub fn generate(rng: &mut SmallRng) -> Question {
    const WEIGHT: [usize; 2] = [
        arithmetic::NUMBER_OF_QUESTION,
        geometry::NUMBER_OF_QUESTION,
    ];

    match rng.random_range(0..WEIGHT.iter().sum()) {
        n if n < arithmetic::NUMBER_OF_QUESTION => {
            arithmetic::generate(rng)
        }
        _ => geometry::generate(rng)
    }
}
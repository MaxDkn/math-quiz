use crate::models::{ Question, Answer };
use crate::tools::{fill_unique_random, format_answers};

use rand::Rng;
use rand::rngs::SmallRng;
use rand::prelude::IndexedRandom;

pub const NUMBER_OF_QUESTION: usize = 1;

const PREFIX_SHAPES: [(&str, usize); 4] = [
    ("pent", 5), ("hex", 6),
    ("oct", 7), ("dec", 10)];
const PREFIX_UNITS: [&str; 7] = ["kilo", "hecto", "deca", "", "deci", "centi", "milli"];

fn q_how_many_side(rng: &mut SmallRng) -> Question {
    const SENTENCES: [&str; 3] = [
        "Combien de côté un ${prefix}agone$ possède t'il ?",
        "Un ${prefix}agone$, c'est un polygone à combien de côté ?",
        "Quel est le nombre de côté d'un ${prefix}agone$ régulier ?"
    ];

    let i = rng.random_range(0..PREFIX_SHAPES.len());
    let (prefix, answer) = PREFIX_SHAPES[i];

    let (min, max) = PREFIX_SHAPES.iter().map(|(_, v)| *v)
        .fold((answer, answer), |(min, max), v|{(min.min(v), max.max(v)) });

    let mut values = vec![answer];
    fill_unique_random(&mut values, 4, rng, min..=max);
    
    let text = SENTENCES.choose(rng)
        .unwrap()
        .replace("{prefix}", prefix);
    
    let answers = format_answers(&values, |v| format!("${v}$"));
    let index_answer = values.iter().position(|&v| v==answer).unwrap();

    Question { text, answers: Answer::Open(answers), index_answer }
}

pub fn generate(rng: &mut SmallRng) -> Question {
    match rng.random_range(0..NUMBER_OF_QUESTION) {
        _ => q_how_many_side(rng),
    }
}

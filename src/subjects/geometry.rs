use crate::models::{ Question, Answer };
use crate::tools::{fill_unique_random, format_answers, pythagorean_triplet};

use rand::Rng;
use rand::rngs::SmallRng;
use rand::prelude::IndexedRandom;

pub const NUMBER_OF_QUESTION: usize = 3;

const PREFIX_SHAPES: [(&str, usize); 6] = [
    ("pent", 5), ("hex", 6), ("hept", 7),
    ("oct", 8), ("ennéa", 9), ("dec", 10)];
const GEOMETRIC_SHAPES_ANGLES : [(&str, (&str, &str)); 3] = [
    ("triangle", ("$180$", "$\\pi$")),
    ("carré", ("$360$", "$2\\pi$")),
    ("pentagone", ("$540$", "$\\dfrac{3\\pi}{2}"))
];
// const PREFIX_UNITS: [&str; 7] = ["kilo", "hecto", "deca", "", "deci", "centi", "milli"];

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

fn q_triangle_nature(rng: &mut SmallRng) -> Question {
    const MIN: usize = 3; const MAX: usize = 10;
    const SENTENCES: [&str; 3] = [
        "Détermine la nature du triangle aux côtés ${a}$, ${b}$, et ${c}$.",
        "À quelle catégorie appartient le triangle avec des côtés de ${a}$, ${b}$ et ${c}$ ?",
        "Identifie la nature du triangle ayant pour côtés ${a}$, ${b}$, et ${c}$."
    ];

    let values: [String; 4] = [
        "rectangle".to_string(), "isocèle".to_string(),
        "équilatéral".to_string(), "quelconque".to_string()
    ];

    let index_answer: usize = rng.random_range(0..4);

    let sides: (usize, usize, usize) = match index_answer {
        0 => {
            pythagorean_triplet(MIN, MAX)
                .choose(rng)
                .copied()
                .expect("No Pythagorean triplets found")
        }
        2 => {
            // triangle équilatéral
            let a = rng.random_range(MIN..=MAX);
            (a, a, a)
        }
        1 => {
            // triangle isocèle
            let a = rng.random_range(MIN..=MAX);
            let b = loop {
                let b = rng.random_range(MIN..=MAX);
                if b != a {
                    break b;
                }
            };
            match rng.random_range(0..3) {
                0 => (a, a, b),
                1 => (a, b, a),
                _ => (b, a, a)
            }

        }
        _ => {
            // triangle quelconque
            let a = rng.random_range(MIN..=MAX);
            let b = loop {
                let b = rng.random_range(MIN..=MAX);
                if b != a {
                    break b;
                }
            };
            let c = loop {
                let c = rng.random_range(MIN..=MAX);
                if c != a && c != b {
                    break c;
                }
            };
            (a, b, c)
        }
    };

    let text = SENTENCES
        .choose(rng)
        .unwrap()
        .replace("{a}", &sides.0.to_string())
        .replace("{b}", &sides.1.to_string())
        .replace("{c}", &sides.2.to_string());

    Question { text, answers: Answer::Open(values), index_answer }
}

fn q_angles_sum(rng: &mut SmallRng) -> Question {
    const SENTENCES: [&str; 3] = [
        "Quelle est la sommes des angles d'un ${shape}$ ?",
        "Quel est le résultat de l’addition des angles d’un ${shape}$ ?",
        "Que vaut la somme des angles d’un ${shape}$ ?"
    ];

    let (shape, (v1, v2)) = GEOMETRIC_SHAPES_ANGLES.choose(rng).unwrap();

    let correct = *[v1, v2].choose(rng).unwrap();
    let forbidden = [v1, v2];

    let mut values = vec![correct];

    while values.len() < 5 {
        let (_, (a, b)) = GEOMETRIC_SHAPES_ANGLES.choose(rng).unwrap();
        let v = *[a, b].choose(rng).unwrap();

        if !values.contains(&v) && !forbidden.contains(&v) {
            values.push(v);
        }
    }
    let index_answer = values.iter().position(|&v| v==correct).unwrap();
    let text = SENTENCES.choose(rng).unwrap().replace("{shape}", shape);

    let answers: [String; 4] = values
        .iter()
        .map(|&v| v.to_string())
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();

    Question { text, answers: Answer::Open(answers), index_answer }
}

pub fn generate(rng: &mut SmallRng) -> Question {
    match rng.random_range(0..NUMBER_OF_QUESTION) {
        0 => q_how_many_side(rng),
        1 => q_angles_sum(rng),
        _ => q_triangle_nature(rng)
    }
}

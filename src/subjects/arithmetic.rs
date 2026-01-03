use crate::tools::function::{ fill_unique_random, format_answers, gcd };

use crate::models::{Answer, Question};
use rand::Rng;
use rand::rngs::SmallRng;
use std::ops::Div;
use rand::prelude::IndexedRandom;

fn q_is_square(rng: &mut SmallRng) -> Question {
    const MIN: usize = 9; const MAX: usize = 169; // numbers here has to be a perfect square!
    const SENTENCES: [&str; 3] = [
        "Le nombre ${n}$ est-il un carré parfait ?",
        "${n}$ est-il le carré d'un nombre entier ?",
        "Peut-on écrire ${n}$ comme $k^2$ avec $k\\in\\mathbb{Z}$ ?"
    ];
    const ANSWERS: [bool; 2] = [true, false];

    let index_answer: usize = rng.random_range(0..=1);

    let num = if ANSWERS[index_answer] {
        let k = rng.random_range(MIN.isqrt()..=MAX.isqrt());
        k * k
    } else {
        loop {
            let candidate = rng.random_range(MIN..=MAX);
            let root = candidate.isqrt();
            if root * root != candidate {
                break candidate;
            }
        }
    };

    let text = SENTENCES
        .choose(rng)
        .unwrap()
        .replace("{n}", &num.to_string());
    Question { text, answers: Answer::Close(ANSWERS), index_answer }
}

fn q_is_prime(rng: &mut SmallRng) -> Question {
    const MIN: usize = 4; const MAX: usize = 40;
    const SENTENCES: [&str; 3] = [
        "${n}$ est-il divisible uniquement par $1$ et lui-même ?",
        "Est-ce que ${n}$ est un nombre premier ?",
        "Le nombre ${n}$ est-il un nombre premier ?"
    ];
    const ANSWERS: [bool; 2] = [true, false];
    const PRIME_NUMBERS: [usize; 10] = [5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let index_answer: usize = rng.random_range(0..=1);

    let num: usize = if ANSWERS[index_answer] {
        *PRIME_NUMBERS.choose(rng).unwrap() } else {
        loop {
            let candidate = rng.random_range(MIN..=MAX);
            if !PRIME_NUMBERS.contains(&candidate) {
                break candidate;
            }
        }
    };

    let text = SENTENCES
        .choose(rng)
        .unwrap()
        .replace("{n}", num.to_string().as_str());

    Question { text, answers: Answer::Close(ANSWERS), index_answer }
}

fn q_div_rem(rng: &mut SmallRng) -> Question {
    const MIN: usize = 10; const MAX: usize = 70;
    const MIN_DIVISOR: usize = 4; const MAX_DIVISOR: usize = 9;
    const SENTENCES: [&str; 2] = [
        "Quel est le reste de la division euclidienne de ${dividend}$ par ${divisor}$ ?",
        "Que vaut $x$ dans ${dividend}\\equiv x\\pmod{{divisor}}$.",
    ];

    let dividend = rng.random_range(MIN..=MAX);
    let divisor = rng.random_range(MIN_DIVISOR..=MAX_DIVISOR);

    let answer = dividend % divisor;

    // Génération des réponses
    let mut values = vec![answer];
    fill_unique_random(&mut values, 4,  rng,
                       0..divisor);

    let answers: [String; 4] = format_answers(&values, |v| format!("${v}$"));

    let text = SENTENCES
        .choose(rng)
        .unwrap()
        .replace("{dividend}", &dividend.to_string())
        .replace("{divisor}", &divisor.to_string());

    let index_answer = values.iter().position(|&v| v == answer).unwrap();

    Question { text, answers: Answer::Open(answers), index_answer }

}

fn q_is_divisible(rng: &mut SmallRng) -> Question {
    const MIN: usize = 100; const MAX: usize = 10_000;
    const DIVISORS: [usize; 7] = [3, 5, 6, 7, 9, 10, 15];
    const SENTENCES: [&str; 3] = [
        "Le nombre ${k}$ divise-t-il ${num}$ ?",
        "Le reste de la division euclidienne de ${num}$ par ${k}$ est-il nul ?",
        "${num}$ est-il divisible par ${k}$ ?"
    ];
    const ANSWERS: [bool; 2] = [true, false];

    let index_answer: usize = rng.random_range(0..=1);
    let divisor = DIVISORS[rng.random_range(0..7)];

    let value = if ANSWERS[index_answer] {
        rng.random_range(MIN.div(divisor)..=MAX.div(divisor)) * divisor
    } else {
        loop {
            let value = rng.random_range(MIN..=MAX);
            if !value.is_multiple_of(divisor) {
                break value;
            }
        }
    };

    let text = SENTENCES
        .choose(rng)
        .unwrap()
        .replace("{num}", &value.to_string())
        .replace("{k}", &divisor.to_string());

    Question { text, answers: Answer::Close(ANSWERS), index_answer }

}

fn q_gcd(rng: &mut SmallRng) -> Question {
    const MIN: usize = 20; const MAX: usize = 60;
    const MIN_SOLUTION: usize = 1; const MAX_SOLUTION: usize = 6;
    const SENTENCES: [&str; 3] = [
        "Trouve le plus grand diviseur commun de ${a}$ et ${b}$.",
        "Quel est le PGCD de ${a}$ et ${b}$ ?",
        "Combien vaut $\\mathrm{pgcd}({a}, {b})$ ?"
    ];

    let k = rng.random_range(MIN_SOLUTION..=MAX_SOLUTION);

    let min_xy = (MIN + k - 1) / k;
    let max_xy = MAX / k;

    let x = rng.random_range(min_xy..=max_xy);
    let co_primes: Vec<usize> = (min_xy..=max_xy)
        .filter(|&y| gcd(x, y) == 1)
        .collect();

    let y = *co_primes.choose(rng).unwrap();

    let (a, b) = (x*k, y*k);

    let mut answers = vec![k];
    fill_unique_random(&mut answers, 4,
                       rng,
                       MIN_SOLUTION..=MAX_SOLUTION);

    let index_answer = answers.iter().position(|&v| { v==k}).unwrap();

    let text = SENTENCES
        .choose(rng)
        .unwrap()
        .replace("{a}", &a.to_string())
        .replace("{b}", &b.to_string());
    let answers: [String; 4] = format_answers(&answers, |v| format!("${v}$"));

    Question { text, answers: Answer::Open(answers), index_answer}
}

fn q_conv_dec(rng: &mut SmallRng) -> Question {
    const MIN: usize = 5; const MAX: usize = 31;
    const SENTENCES: [&str; 3] = [
        "Transforme le nombre ${n}$ en nombre binaire.",
        "Exprime ${n}$ en base $2$.",
        "Convertis ${n}$ du décimal vers le binaire."
    ];

    let value = rng.random_range(MIN..=MAX);
    let mut values = vec![value];

    fill_unique_random(&mut values, 4, rng, MIN..=MAX);

    let answers: [String; 4] = format_answers(&values, |v| format!("${:05b}_2$", v));

    let text = SENTENCES
        .choose(rng)
        .unwrap()
        .replace("{n}", &value.to_string());

    let index_answer = values.iter().position(|&v| v == value).unwrap();
    Question { text, answers: Answer::Open(answers), index_answer }
}

fn q_conv_bin(rng: &mut SmallRng) -> Question {
    const MIN: usize = 5; const MAX: usize = 31;
    const SENTENCES: [&str; 3] = [
        "Transforme le nombre ${n}_2$ en nombre décimal.",
        "Exprime ${n}_2$ en base $10$.",
        "Convertis ${n}_2$ du binaire vers le décimal."
    ];


    let value = rng.random_range(MIN..=MAX);
    let mut values = vec![value];

    fill_unique_random(&mut values, 4, rng, MIN..=MAX);

    let answers: [String; 4] = format_answers(&values, |v| format!("${v}$"));

    let text = SENTENCES
        .choose(rng)
        .unwrap()
        .replace("{n}", &format!("{:05b}", value));

    let index_answer = values.iter().position(|&v| v == value).unwrap();
    Question { text, answers: Answer::Open(answers), index_answer }
}

pub fn generate(rng: &mut SmallRng) -> Question {
    let x: bool = rng.random();

    match rng.random_range(0..6) {
        0 => q_is_square(rng),
        1 => q_is_prime(rng),
        2 => q_div_rem(rng),
        3 => q_is_divisible(rng),
        4 => q_gcd(rng),
        _ => if x { q_conv_dec(rng) } else { q_conv_bin(rng) }
    }
}
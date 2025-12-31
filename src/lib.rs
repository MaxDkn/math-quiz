mod models;

use rand::Rng;
use rand::SeedableRng;
use std::cell::RefCell;
use rand::rngs::SmallRng;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use models::{Question, Answers};

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(42));
}

#[wasm_bindgen]
pub fn init_rng(seed: u64) {
	RNG.with(|rng| {
		*rng.borrow_mut() = SmallRng::seed_from_u64(seed);
	});
}

#[wasm_bindgen]
pub fn reverse(word: &str) -> String {
	let reversed_word: String = word.chars().rev().collect();
	format!("The reverse of {} is {}", word, reversed_word)
}

#[wasm_bindgen]
pub fn random_numbers(count: usize) -> Vec<u32> {
	RNG.with(|rng| {
		let mut r = rng.borrow_mut();
		(0..count).map(|_| r.gen_range(0..100)).collect()
	})
}

#[wasm_bindgen]
pub fn generate() -> Result<JsValue, JsValue> {
	let question = Question {
		text: "$7$ divise-t'il $63$ ?".to_string(),
		answers: Answers::Close([true, false]),
		index_answer: 0,
	};
	Ok(serde_wasm_bindgen::to_value(&question)?)
}

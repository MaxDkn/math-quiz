mod models;

// use rand::Rng;
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

// #[wasm_bindgen]
// pub fn random_numbers(count: usize) -> Vec<u32> {
// 	RNG.with(|rng| {
// 		let mut r = rng.borrow_mut();
// 		(0..count).map(|_| r.random_range(0..100)).collect()
// 	})
// }

#[wasm_bindgen]
pub fn generate() -> Result<JsValue, JsValue> {
	let question = Question {
		text: "$7$ divise-t'il $63$ ?".to_string(),
		answers: Answers::Open(["true".to_string(), "false".to_string(), "$1$".to_string(), "$2$".to_string()]),
		index_answer: 0,
	};
	Ok(serde_wasm_bindgen::to_value(&question)?)
}

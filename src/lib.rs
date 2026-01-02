mod models;
mod subjects;

// use rand::Rng;
use rand::SeedableRng;
use std::cell::RefCell;
use rand::rngs::SmallRng;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use crate::subjects::arithmetic;

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
pub fn generate() -> Result<JsValue, JsValue> {
	let question = &arithmetic::generate();
	Ok(serde_wasm_bindgen::to_value(&question)?)
}

// #[wasm_bindgen]
// pub fn random_numbers(count: usize) -> Vec<u32> {
// 	RNG.with(|rng| {
// 		let mut r = rng.borrow_mut();
// 		(0..count).map(|_| r.random_range(0..100)).collect()
// 	})
// }
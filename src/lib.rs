mod tools;
mod models;
mod subjects;

use rand::SeedableRng;
use std::cell::RefCell;
use serde_wasm_bindgen;
use rand::rngs::SmallRng;
use wasm_bindgen::prelude::*;

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
	RNG.with(|rng| {
		let question = subjects::generate(&mut rng.borrow_mut());
		Ok(serde_wasm_bindgen::to_value(&question)?)
	})
}
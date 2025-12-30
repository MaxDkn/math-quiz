use wasm_bindgen::prelude::*;
use rand::Rng;
use rand::rngs::SmallRng;
use std::cell::RefCell;
use rand::SeedableRng;

const SEED: u64 = 42;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(SEED));
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

pub mod ledger;
pub mod utils;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;

thread_local!(pub static ONCHAIN_MAP_SEED: RefCell<u32> = RefCell::new(0));


#[wasm_bindgen]
pub fn set_map_randomness(seed: u32) {
    ONCHAIN_MAP_SEED.with(|s| {
        *s.borrow_mut() = seed;
    });
}

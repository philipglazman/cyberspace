use std::cell::RefCell;

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub address: String,
    pub score: String,
}

thread_local!(pub static ECONOMIC_VICTORY: RefCell<Vec<Player>> = RefCell::new(Vec::new()));
thread_local!(pub static CULTURAL_VICTORY: RefCell<Vec<Player>> = RefCell::new(Vec::new()));
thread_local!(pub static DIPLOMATIC_VICTORY: RefCell<Vec<Player>> = RefCell::new(Vec::new()));

#[wasm_bindgen]
pub fn set_economic_victory_leaderboard(players: Vec<JsValue>) {
    ECONOMIC_VICTORY.with(|s| {
        let mut res = Vec::new();
        for player in players {
            let p: Player = serde_wasm_bindgen::from_value(player).expect("parse player");
            res.push(p)
        }
        
        *s.borrow_mut() = res;
    });
}

#[wasm_bindgen]
pub fn set_cultural_victory_leaderboard(players: Vec<JsValue>) {
    CULTURAL_VICTORY.with(|s| {
        let mut res = Vec::new();
        for player in players {
            let p: Player = serde_wasm_bindgen::from_value(player).expect("parse player");
            res.push(p)
        }
        
        *s.borrow_mut() = res;
    });
}

#[wasm_bindgen]
pub fn set_diplomatic_victory_leaderboard(players: Vec<JsValue>) {
    DIPLOMATIC_VICTORY.with(|s| {
        let mut res = Vec::new();
        for player in players {
            let p: Player = serde_wasm_bindgen::from_value(player).expect("parse player");
            res.push(p)
        }
        
        *s.borrow_mut() = res;
    });
}
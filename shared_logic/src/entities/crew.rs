use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrewMember {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub movement_speed: f32, 
}

#[wasm_bindgen]
impl CrewMember {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, x: f32, y: f32, movement_speed: f32) -> Self {
        Self {
            id,
            x,
            y,
            movement_speed,
        }
    }
}
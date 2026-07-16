pub fn calculate_level_from_xp(xp: u32) -> u32 {
    let raw_level = (xp as f64).sqrt() / 10.0;
    
    raw_level as u32 + 1
}

#[cfg(target_arch = "wasm32")]
pub mod wasm_bindings {
    use super::*; 
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen(js_name = calculateLevelFromXp)]
    pub fn js_calculate_level_from_xp(xp: u32) -> u32 {
        calculate_level_from_xp(xp)
    }
}
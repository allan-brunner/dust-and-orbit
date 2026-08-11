use wasm_bindgen::prelude::wasm_bindgen;

use crate::entities::crew::CrewMember;
use crate::world::SurfaceDebris;
use crate::world::chunk::{Chunk, CHUNK_SIZE};

pub const TIME_PER_PEBBLE_SEC: f32 = 0.5;

#[wasm_bindgen]
pub fn pebble_gathering_time(
    crew: &CrewMember,
    chunk: &Chunk,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> f32 {
    let mut total_time_seconds: f32 = 0.0;

    let mut current_x = crew.x;
    let mut current_y = crew.y;

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            let index = y * CHUNK_SIZE + x;

            let Some(tile) = chunk.tiles.get(index) else {
                continue;
            };

            if let SurfaceDebris::Pebbles { count } = tile.surface {
                if count > 0 {
                    let target_x = x as f32 + 0.5;
                    let target_y = y as f32 + 0.5;

                    let dx = target_x - current_x;
                    let dy = target_y - current_y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    let travel_time = distance / crew.movement_speed;
                    total_time_seconds += travel_time;

                    let gather_time = (count as f32) * TIME_PER_PEBBLE_SEC;
                    total_time_seconds += gather_time;

                    current_x = target_x;
                    current_y = target_y;
                }
            }
        }
    }

    total_time_seconds
}
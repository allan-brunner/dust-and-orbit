use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::world::{FoliageType, MaterialType, PlantEntity, SurfaceDebris, VerticalSpan, WorldTileColumn};

pub const CHUNK_SIZE: usize = 64;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl ChunkCoord {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub(crate) plants: Vec<PlantEntity>,
    pub(crate) tiles: Vec<WorldTileColumn>,
}

/// A simple, deterministic pseudo-random hash function.
/// Returns a float between 0.0 and 1.0 based on x, y, and a seed.
fn pseudo_random(x: i32, y: i32, seed: i32) -> f32 {
    let mut n = x.wrapping_mul(331)
        .wrapping_add(y.wrapping_mul(733))
        .wrapping_add(seed.wrapping_mul(139));
        
    n ^= n << 13;
    n = n.wrapping_mul(n.wrapping_mul(n).wrapping_mul(15731).wrapping_add(789221)).wrapping_add(1376312589);
    
    // Mask to positive and divide by max i32 to get 0.0 - 1.0
    ((n as u32 & 0x7fffffff) as f32) / 2147483648.0
}

/// Generates a terraced heightmap using interfering sine waves.
fn get_elevation(global_x: f32, global_y: f32) -> i32 {
    // Combine different frequencies and amplitudes for organic looking hills
    let wave1 = (global_x / 14.0).sin() * (global_y / 14.0).cos() * 5.0;
    let wave2 = ((global_x + global_y) / 22.0).sin() * 3.0;
    
    let raw_height = wave1 + wave2;
    
    // .round() creates the distinct, flat "steps" and cliffs used in isometric views
    raw_height.round() as i32
}

#[wasm_bindgen]
impl Chunk {
    pub fn new_deterministic(coord: ChunkCoord) -> Self {
        let mut tiles = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE);
        let mut plants = Vec::new();
        let mut plant_id = 0;

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                // Calculate global coordinates for seamless noise across chunk borders
                let global_x = (coord.x * CHUNK_SIZE as i32) + x as i32;
                let global_y = (coord.y * CHUNK_SIZE as i32) + y as i32;

                // Determine Elevation
                let height = get_elevation(global_x as f32, global_y as f32);

                // Determine Top Material
                // If it's a deep hole, expose Stone. Otherwise, use Dirt.
                let top_material = if height < -3 { MaterialType::Stone } else { MaterialType::Dirt };

                // Scatter Surface Debris using our PRNG hash
                let debris_hash = pseudo_random(global_x, global_y, 1);
                let surface = if debris_hash > 0.96 {
                    SurfaceDebris::LargeRock
                } else if debris_hash > 0.85 {
                    // Randomize pebble count between 1 and 4
                    let count = ((debris_hash * 100.0) as u8 % 4) + 1;
                    SurfaceDebris::Pebbles { count }
                } else {
                    SurfaceDebris::None
                };

                // Build the column with bedrock at the bottom, stone in the middle, and top material
                tiles.push(WorldTileColumn {
                    surface,
                    vertical_spans: vec![
                        VerticalSpan {
                            start_y: -64,
                            end_y: -20,
                            material: MaterialType::Bedrock,
                            integrity: 255,
                        },
                        VerticalSpan {
                            start_y: -20,
                            end_y: height - 1,
                            material: MaterialType::Stone,
                            integrity: 200,
                        },
                        VerticalSpan {
                            start_y: height - 1,
                            end_y: height,
                            material: top_material,
                            integrity: 100,
                        },
                    ],
                });

                // Scatter Plants
                let plant_hash = pseudo_random(global_x, global_y, 2);
                
                // Only spawn plants on dirt, where there is no debris, based on chance
                if top_material == MaterialType::Dirt && surface == SurfaceDebris::None && plant_hash > 0.88 {
                    let is_tree = plant_hash > 0.95; // 5% chance of bush, ~2% chance of tree
                    
                    plants.push(PlantEntity {
                        id: plant_id,
                        local_anchor_x: x as u8,
                        local_anchor_y: y as u8,
                        trunk_size: if is_tree { 3 } else { 0 },
                        height: if is_tree { 15 } else { 4 },
                        clearance_radius: if is_tree { 1.5 } else { 0.5 },
                        foliage_type: if is_tree { FoliageType::LargeTree } else { FoliageType::SmallBush },
                        health: 255,
                    });
                    
                    plant_id += 1;
                }
            }
        }

        Self {
            coord,
            plants,
            tiles,
        }
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(self).map_err(|err| JsValue::from_str(&err.to_string()))
    }
}
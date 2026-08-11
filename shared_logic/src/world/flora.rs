use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FoliageType {
    SmallBush,
    LargeTree,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlantEntity {
    pub id: u16,
    pub local_anchor_x: u8,
    pub local_anchor_y: u8,
    pub trunk_size: u8,
    pub height: u8,
    pub clearance_radius: f32,
    pub foliage_type: FoliageType,
    pub health: u8,
}
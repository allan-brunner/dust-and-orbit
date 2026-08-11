use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SurfaceDebris {
    None,
    Pebbles { count: u8 },
    LargeRock,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MaterialType {
    Empty,
    Dirt,
    Stone,
    Bedrock,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct VerticalSpan {
    pub start_y: i32,
    pub end_y: i32,
    pub material: MaterialType,
    pub integrity: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldTileColumn {
    pub surface: SurfaceDebris,
    pub vertical_spans: Vec<VerticalSpan>,
}
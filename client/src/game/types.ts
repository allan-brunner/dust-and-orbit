export interface ChunkCoord {
    x: number;
    y: number;
}

export type SurfaceDebris = 'None' | { Pebbles: { count: number } } | 'LargeRock';

export type MaterialType = 'Empty' | 'Dirt' | 'Stone' | 'Bedrock';

export interface VerticalSpan {
    start_y: number;
    end_y: number;
    material: MaterialType;
    integrity: number;
}

export interface WorldTileColumn {
    surface: SurfaceDebris;
    vertical_spans: VerticalSpan[];
}

export type FoliageType = 'SmallBush' | 'LargeTree';

export interface PlantEntity {
    id: number;
    local_anchor_x: number;
    local_anchor_y: number;
    trunk_size: number;
    height: number;
    clearance_radius: number;
    foliage_type: FoliageType;
    health: number; // 0-255
}

export const CHUNK_SIZE = 64;

export interface Chunk {
    coord: ChunkCoord;
    plants: PlantEntity[];
    tiles: WorldTileColumn[];
}
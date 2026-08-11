import { CHUNK_SIZE, type Chunk, type MaterialType, type VerticalSpan, type WorldTileColumn } from './types';

export const ISO = {
    tileWidth: 64,
    tileHeight: 32,
    pixelsPerHeight: 6,
} as const;

// A dictionary mapping for faster lookups and cleaner code.
const MATERIAL_COLORS: Record<MaterialType, number> = {
    Dirt: 0x8b5a2b,
    Stone: 0x8a8a8e,
    Bedrock: 0x37373c,
    Empty: 0x0a0a12,
};

/**
 * Flattens 2D coordinates into a 1D array index.
 */
export function tileIndex(x: number, y: number): number {
    return x + y * CHUNK_SIZE;
}

/**
 * Safely fetches a tile from the chunk, preventing out-of-bounds errors.
 */
export function getTile(chunk: Chunk, x: number, y: number): WorldTileColumn | undefined {
    if (x < 0 || y < 0 || x >= CHUNK_SIZE || y >= CHUNK_SIZE) return undefined;
    return chunk.tiles[tileIndex(x, y)];
}

/**
 * Finds the highest non-empty material block in a tile column.
 */
export function getSurfaceSpan(tile: WorldTileColumn): VerticalSpan | undefined {
    return tile.vertical_spans
        .filter(span => span.material !== 'Empty')
        .reduce<VerticalSpan | undefined>((highest, current) => {
            return (!highest || current.end_y > highest.end_y) ? current : highest;
        }, undefined);
}

/**
 * Helper to quickly grab the top Y-elevation of a tile.
 */
export function getSurfaceHeight(tile: WorldTileColumn): number {
    return getSurfaceSpan(tile)?.end_y ?? 0;
}

/**
 * Core Isometric Math: Converts grid (x, y) to screen pixels.
 */
export function gridToScreen(gx: number, gy: number): { x: number; y: number } {
    return {
        x: (gx - gy) * (ISO.tileWidth / 2),
        y: (gx + gy) * (ISO.tileHeight / 2),
    };
}

/**
 * Converts elevation units into vertical screen pixels.
 */
export function elevationOffset(height: number): number {
    return height * ISO.pixelsPerHeight;
}

/**
 * Maps a material type to its base hex color.
 */
export function materialColor(material: MaterialType): number {
    return MATERIAL_COLORS[material] ?? MATERIAL_COLORS.Empty;
}
import { Container, Graphics } from 'pixi.js';
import { CHUNK_SIZE, type Chunk, type PlantEntity, type SurfaceDebris } from './types';
import {
    ISO,
    elevationOffset,
    getSurfaceHeight,
    getSurfaceSpan,
    getTile,
    gridToScreen,
    materialColor,
} from './iso';

const COLORS = {
    ROCK: 0x5b5b60,
    ROCK_STROKE: 0x2c2c30,
    PEBBLE: 0x9a9a92,
    TREE_TRUNK: 0x4a3524,
    TREE_CANOPY: 0x2f6b3a,
    BUSH_CANOPY: 0x4f8f4a
};

/**
 * A fast, deterministic Pseudo-Random Number Generator (PRNG).
 */
function hash01(seed: number): number {
    const s = Math.sin(seed * 12.9898) * 43758.5453;
    return s - Math.floor(s);
}

/**
 * Adjusts the brightness of a hex color.
 */
function shade(color: number, factor: number): number {
    const r = (color >> 16) & 0xff;
    const g = (color >> 8) & 0xff;
    const b = color & 0xff;
    const clamp = (c: number) => Math.max(0, Math.min(255, Math.round(c)));
    const mix = (c: number) => (factor < 0 ? c * (1 + factor) : c + (255 - c) * factor);
    return (clamp(mix(r)) << 16) | (clamp(mix(g)) << 8) | clamp(mix(b));
}

export class ChunkRenderer {
    readonly view = new Container();

    // Separate terrain, debris, and plants into distinct layers (Graphics objects)
    // so PixiJS can optimize batch rendering for them.
    private readonly terrain = new Graphics();
    private readonly debris = new Graphics();
    private readonly plants = new Container();

    constructor() {
        this.view.addChild(this.terrain, this.debris, this.plants);
    }

    render(chunk: Chunk): void {
        if (chunk.tiles.length !== CHUNK_SIZE * CHUNK_SIZE) return;

        // Clear previous frame data. 
        this.terrain.clear();
        this.debris.clear();
        this.plants.removeChildren();

        // THE PAINTER'S ALGORITHM:
        // In isometric space, tiles with a lower (x + y) sum are further "back".
        // We iterate by 'sum' so we draw back-to-front, ensuring proper overlap.
        const maxSum = (CHUNK_SIZE - 1) * 2;
        for (let sum = 0; sum <= maxSum; sum++) {
            const gyStart = Math.max(0, sum - (CHUNK_SIZE - 1));
            const gyEnd = Math.min(CHUNK_SIZE - 1, sum);
            for (let gy = gyStart; gy <= gyEnd; gy++) {
                this.drawTile(chunk, sum - gy, gy);
            }
        }

        // Plants are added last as they sit on top of the terrain.
        for (const plant of chunk.plants) {
            this.plants.addChild(this.buildPlant(plant, chunk));
        }
    }

    destroy(): void {
        this.view.destroy({ children: true });
    }

    private drawTile(chunk: Chunk, gx: number, gy: number): void {
        const tile = getTile(chunk, gx, gy);
        if (!tile) return;

        const span = getSurfaceSpan(tile);
        const height = span?.end_y ?? 0;
        const color = materialColor(span?.material ?? 'Empty');

        // Damage darkens the tile
        const integrity = span?.integrity ?? 255;
        const damage = 1 - integrity / 255;

        // Base screen coordinate for the center of the tile
        const { x, y } = gridToScreen(gx, gy);
        const hw = ISO.tileWidth / 2;
        const hh = ISO.tileHeight / 2;

        // Offset Y by the tile's physical elevation
        const topY = y - elevationOffset(height);

        // Draw the top diamond of the tile
        this.terrain
            .poly([x, topY - hh, x + hw, topY, x, topY + hh, x - hw, topY])
            .fill(shade(color, -0.35 * damage));

        // CLIFF DRAWING:
        // To give the illusion of 3D, we look at the tiles immediately East (gx+1) 
        // and South (gy+1). If they are lower than this tile, we draw a vertical "cliff" wall.
        const east = getTile(chunk, gx + 1, gy);
        if (east) {
            const eastHeight = getSurfaceHeight(east);
            const eastColor = materialColor(getSurfaceSpan(east)?.material ?? 'Empty');
            this.drawCliff(x + hw, y, x, y + hh, height, eastHeight, height >= eastHeight ? color : eastColor, -0.25);
        }

        const south = getTile(chunk, gx, gy + 1);
        if (south) {
            const southHeight = getSurfaceHeight(south);
            const southColor = materialColor(getSurfaceSpan(south)?.material ?? 'Empty');
            this.drawCliff(x, y + hh, x - hw, y, height, southHeight, height >= southHeight ? color : southColor, -0.45);
        }

        this.drawDebris(x, topY, gx, gy, tile.surface);
    }

    /**
     * Draws the vertical walls bridging two tiles of different elevations.
     */
    private drawCliff(vAx: number, vAy: number, vBx: number, vBy: number, heightA: number, heightB: number, baseColor: number, shadeFactor: number): void {
        if (heightA === heightB) return;
        const highOff = elevationOffset(Math.max(heightA, heightB));
        const lowOff = elevationOffset(Math.min(heightA, heightB));

        // Connects the top points and bottom points to form the wall
        this.terrain
            .poly([vAx, vAy - highOff, vBx, vBy - highOff, vBx, vBy - lowOff, vAx, vAy - lowOff])
            .fill(shade(baseColor, shadeFactor));
    }

    private drawDebris(cx: number, cy: number, gx: number, gy: number, surface: SurfaceDebris): void {
        if (surface === 'None') return;

        if (surface === 'LargeRock') {
            const wobble = hash01(gx * 131 + gy * 977) * 4 - 2;
            this.debris
                .poly([cx - 8, cy + wobble, cx, cy - 10, cx + 9, cy + wobble + 2, cx + 3, cy + 8, cx - 5, cy + 7])
                .fill(COLORS.ROCK)
                .stroke({ width: 1, color: COLORS.ROCK_STROKE, alpha: 0.6 });
            return;
        }

        const count = surface.Pebbles.count;
        for (let i = 0; i < count; i++) {
            const seed = gx * 733 + gy * 197 + i * 53;
            const ox = (hash01(seed) - 0.5) * ISO.tileWidth * 0.6;
            const oy = (hash01(seed + 1) - 0.5) * ISO.tileHeight * 0.6;
            const r = 1.5 + hash01(seed + 2) * 1.5;
            this.debris.circle(cx + ox, cy + oy, r).fill(COLORS.PEBBLE);
        }
    }

    private buildPlant(plant: PlantEntity, chunk: Chunk): Graphics {
        const { local_anchor_x: gx, local_anchor_y: gy } = plant;
        const tile = getTile(chunk, gx, gy);
        const height = tile ? getSurfaceHeight(tile) : 0;
        const { x, y } = gridToScreen(gx, gy);
        const baseY = y - elevationOffset(height);

        // Plants get a slight positional jitter so they don't look like an artificial grid
        const jitterX = (hash01(plant.id * 17 + 1) - 0.5) * ISO.tileWidth * 0.5;
        const jitterY = (hash01(plant.id * 31 + 2) - 0.5) * ISO.tileHeight * 0.5;

        const isTree = plant.foliage_type === 'LargeTree';
        const healthFactor = plant.health / 255;
        const canopyColor = shade(isTree ? COLORS.TREE_CANOPY : COLORS.BUSH_CANOPY, (healthFactor - 1) * 0.6);
        const canopyRadius = (isTree ? 10 : 5) + plant.height * (isTree ? 0.4 : 0.3);
        const trunkHeight = Math.max(4, plant.trunk_size);

        const g = new Graphics();

        // Negative Y is "up" on the screen, so we draw from the anchor upwards
        if (isTree) {
            g.rect(-1.5, -trunkHeight, 3, trunkHeight).fill(COLORS.TREE_TRUNK);
        }
        g.circle(0, isTree ? -(trunkHeight + canopyRadius * 0.6) : -canopyRadius * 0.5, canopyRadius).fill(canopyColor);

        g.position.set(x + jitterX, baseY + jitterY);
        return g;
    }
}
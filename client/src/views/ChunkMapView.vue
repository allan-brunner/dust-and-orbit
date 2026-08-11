<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch, shallowRef } from 'vue';
import { Application } from 'pixi.js';
import { ChunkRenderer } from '../game/ChunkRenderer';
import type { Chunk as ChunkShape } from '../game/types';
import initWasm, { Chunk, ChunkCoord } from '../../../shared_logic/pkg/shared_logic';

const props = defineProps<{ chunk?: Chunk }>();
const canvasHost = ref<HTMLDivElement | null>(null);

const app = shallowRef<Application | null>(null);
const renderer = shallowRef<ChunkRenderer | null>(null);

onMounted(async () => {
    if (!canvasHost.value) return;

    app.value = new Application();
    await app.value.init({
        background: '#101018',
        antialias: true,
        resizeTo: canvasHost.value,
    });

    canvasHost.value.appendChild(app.value.canvas);

    renderer.value = new ChunkRenderer();
    renderer.value.view.position.set(app.value.screen.width / 2, 80);
    app.value.stage.addChild(renderer.value.view);

    await initWasm();
    
    // Test render a deterministic chunk
    const chunk = Chunk.new_deterministic(new ChunkCoord(0, 0));
    renderer.value.render(chunk.toJSON() as ChunkShape);

    enablePan(app.value);
});

watch(
    () => props.chunk,
    (newChunk) => {
        if (newChunk && renderer.value) {
            renderer.value.render(newChunk.toJSON() as ChunkShape);
        }
    }
);

onBeforeUnmount(() => {
    if (app.value) {
        app.value.destroy(true, { children: true });
    }
    app.value = null;
    renderer.value = null;
});

/**
 * Attaches pointer events for drag-to-pan camera mechanics.
 */
function enablePan(pixiApp: Application): void {
    const stage = pixiApp.stage;
    stage.eventMode = 'static';
    stage.hitArea = pixiApp.screen;

    let dragging = false;
    let last = { x: 0, y: 0 };

    stage.on('pointerdown', (e) => {
        dragging = true;
        last = { x: e.global.x, y: e.global.y };
    });

    const stopDrag = () => { dragging = false; };
    stage.on('pointerup', stopDrag);
    stage.on('pointerupoutside', stopDrag);

    stage.on('pointermove', (e) => {
        if (!dragging || !renderer.value) return;
        
        // Delta pan logic
        renderer.value.view.x += e.global.x - last.x;
        renderer.value.view.y += e.global.y - last.y;
        last = { x: e.global.x, y: e.global.y };
    });
}
</script>

<template>
    <div ref="canvasHost" class="chunk-map"></div>
</template>

<style scoped>
.chunk-map {
    width: 100%;
    height: 100%;
    min-height: 480px;
    overflow: hidden;
    cursor: grab;
}
.chunk-map:active {
    cursor: grabbing;
}
</style>
<script setup lang="ts">
import ChunkMapView from './ChunkMapView.vue';
</script>

<template>
  <div class="game-layout">
    <div class="canvas-layer">
      <ChunkMapView />
    </div>

    <div class="ui-layer">
      
      <header class="top-bar">
        <span>Dust & Orbit</span>
        <router-link to="/" class="btn-exit">Exit Game</router-link>
      </header>

      <aside class="inventory-panel">
        <h3>Inventory</h3>
        <div class="inventory-grid">
          <div class="slot"></div>
          <div class="slot"></div>
          <div class="slot"></div>
          <div class="slot"></div>
        </div>
      </aside>

    </div>
  </div>
</template>

<style scoped>
/* The main wrapper takes up the whole screen without scrolling */
.game-layout {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden; /* Prevents scrollbars from appearing */
  background: #101018; /* Matches your canvas background */
}

/* Canvas sits at the very bottom (z-index 0) */
.canvas-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
}

/* UI layer sits on top, but uses pointer-events: none so clicks pass 
   through to the canvas unless they hit a specific UI element. */
.ui-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 10;
  pointer-events: none; 
}

/* --- UI Element Styling --- */

.top-bar {
  pointer-events: auto; /* Re-enable clicking for this specific element */
  display: flex;
  justify-content: space-between;
  padding: 15px 20px;
  background: rgba(16, 16, 24, 0.8);
  border-bottom: 1px solid var(--border);
  color: white;
}

.inventory-panel {
  pointer-events: auto; /* Re-enable clicking so you can click items */
  position: absolute;
  right: 20px;
  top: 80px;
  width: 250px;
  background: rgba(22, 23, 29, 0.9);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 15px;
  color: white;
  box-shadow: var(--shadow);
}

.inventory-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
  margin-top: 10px;
}

.slot {
  aspect-ratio: 1;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--border);
  border-radius: 4px;
}

.btn-exit {
  color: #ff5c5c;
  text-decoration: none;
  font-weight: bold;
}
</style>
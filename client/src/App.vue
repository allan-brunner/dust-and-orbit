<script setup lang="ts">
import { ref, onMounted } from 'vue'
import initWasm, { calculateLevelFromXp } from '../../shared_logic/pkg/shared_logic.js'

const serverStatus = ref<any>(null)
const error = ref<string | null>(null)
const playerXp = ref(4500)
const clientCalculatedLevel = ref(0)

onMounted(async () => {
  await initWasm()
  clientCalculatedLevel.value = calculateLevelFromXp(playerXp.value)

  try {
    const response = await fetch('/api/status')
    
    if (!response.ok) {
      throw new Error(`Server returned HTTP ${response.status}`)
    }

    serverStatus.value = await response.json()
  } catch (e: any) {
    error.value = "Unable to connect to the game server. Is the backend running?"
    console.error(e)
  }
})
</script>

<template>
  <div class="container">
    <h1>Dust & Orbit Connection Test</h1>

    <div v-if="error" class="card error">
      <h3>Connection Error</h3>
      <p>{{ error }}</p>
    </div>

    <div v-else-if="serverStatus" class="card success">
      <h3>Connection Established</h3>
      <p>Server message: <i>"{{ serverStatus.message }}"</i></p>
      
      <div class="db-grid">
        <div class="db-status">
          <strong>PostgreSQL (Market):</strong> 
          <span :class="{ connected: serverStatus.postgres_connected }">
            {{ serverStatus.postgres_connected ? 'Active' : 'Offline' }}
          </span>
        </div>
        <div class="db-status">
          <strong>MongoDB (World state):</strong> 
          <span>Active ({{ serverStatus.mongo_collections_found }} collections found)</span>
        </div>
        <div class="db-status">
          <strong>Player XP (backend):</strong> 
          <span>Level: {{ serverStatus.player_stats.verified_level }} ({{ serverStatus.player_stats.xp }}xp)</span>
        </div>
      </div>
    </div>

    <div v-else class="card loading">
      <p>Initiating handshake with backend...</p>
    </div>

    <br/>

    <div class="card">
      <h2>WASM Instant Calculation</h2>
      <p>Player XP: {{ playerXp }}</p>
      <p>Calculated Level (Frontend): <b>{{ clientCalculatedLevel }}</b></p>
      
      <button @click="playerXp += 500; clientCalculatedLevel = calculateLevelFromXp(playerXp)">
        Add 500 XP
      </button>
    </div>
  </div>
</template>
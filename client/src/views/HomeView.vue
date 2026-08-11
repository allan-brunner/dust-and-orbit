<script setup lang="ts">
import { ref, onMounted } from 'vue'
import initWasm, { calculateLevelFromXp } from '../../../shared_logic/pkg/shared_logic.js'
import { useAuthStore } from '../stores/auth'
import Login from '../components/Login.vue'
import Status from '../components/Status.vue'

const authStore = useAuthStore()

// App state
const isAppReady = ref(false)
const error = ref<string | null>(null)

// Game state test variables
const playerXp = ref(4500)
const clientCalculatedLevel = ref(0)

onMounted(async () => {
  try {
    const [_, __] = await Promise.all([
      authStore.initializeAuth(),
      initWasm()
    ])

    clientCalculatedLevel.value = calculateLevelFromXp(playerXp.value)

  } catch (e: any) {
    error.value = "Unable to connect to the game server. Is the backend running?"
    console.error(e)
  } finally {
    isAppReady.value = true
  }
})
</script>

<template>
  <div class="container">
    <h1>Dust & Orbit</h1>

    <div v-if="!isAppReady" class="card loading">
      <p>Initiating handshake and verifying session...</p>
    </div>

    <div v-else-if="error" class="card error">
      <h3>Connection Error</h3>
      <p>{{ error }}</p>
    </div>

    <div v-else>
      <Status></Status>
      
      <Login v-if="!authStore.isAuthenticated" />

      <div v-else>
        <div class="card success" style="margin-bottom: 20px; text-align: center;">
          <h2>Welcome back, {{ authStore.userUsername }}</h2>
          <button @click="authStore.logout()">Log Out</button>
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
    </div>
  </div>
</template>

<style scoped>
.card {
  padding: 20px;
  border-radius: 8px;
  border: 1px solid #ddd;
  color: #333;
}
.error { border-color: red; background: #ffe6e6; }
.loading { text-align: center; color: #666; font-style: italic; }
.header { text-align: center; margin-bottom: 20px; }
</style>
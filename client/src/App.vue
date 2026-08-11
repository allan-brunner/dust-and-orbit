<script setup lang="ts">
import { ref, onMounted } from 'vue';
import initWasm from '../../shared_logic/pkg/shared_logic.js';
import { useAuthStore } from './stores/auth';

const authStore = useAuthStore();
const isAppReady = ref(false);
const error = ref<string | null>(null);

onMounted(async () => {
  try {
    await Promise.all([
      authStore.initializeAuth(),
      initWasm()
    ]);
  } catch (e: any) {
    error.value = "Unable to connect to the game server. Is the backend running?";
  } finally {
    isAppReady.value = true;
  }
});
</script>

<template>
  <div v-if="!isAppReady" class="global-loading">
    <p>Initiating handshake and verifying session...</p>
  </div>
  <div v-else-if="error" class="global-error">
    <h3>Connection Error</h3>
    <p>{{ error }}</p>
  </div>
  
  <router-view v-else></router-view>
</template>

<style scoped>
.global-loading, .global-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  text-align: center;
}
</style>
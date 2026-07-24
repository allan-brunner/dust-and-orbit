<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { api } from '../api';

const statusData = ref<any>(null);
const frontendVersion = import.meta.env.VITE_APP_VERSION || 'dev'; 

onMounted(async () => {
    try {
        const response = await api.get('/api/status');
        statusData.value = response.data;
    } catch (e) {
        console.error("Failed to load status");
    }
});
</script>

<template>
  <div v-if="statusData" class="status-card">
    <h3>System Status: {{ statusData.status }}</h3>
    
    <ul>
      <li><strong>PostgreSQL:</strong> {{ statusData.db_postgres }}</li>
      <li><strong>MongoDB:</strong> {{ statusData.db_mongo }}</li>
      <li><strong>Active Players:</strong> {{ statusData.active_sessions }}</li>
    </ul>

    <div class="versions">
      <span>UI: v{{ frontendVersion }}</span> | 
      <span>API: v{{ statusData.backend_version }}</span>
    </div>
  </div>
</template>

<style scoped>
ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}
</style>
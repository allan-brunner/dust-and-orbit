<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { api } from '../api';
import pkg from '../../package.json'

const appVersion = pkg.version

const statusData = ref<any>(null);
const apiResponseTime = ref<string>('0 ms');

onMounted(async () => {
    try {
        const start = performance.now();
        const response = await api.get('/admin/status');
        const end = performance.now();

        apiResponseTime.value = end - start + ' ms';
        
        statusData.value = response.data;
    } catch (e) {
        console.error("Failed to load status", e);
    }
});
</script>

<template>
  <div v-if="statusData" class="status-card">
    <h3>System status: {{ statusData.status }}</h3>
    <p><strong>MongoDB:</strong> {{ statusData.db_mongo }}</p>
    <p><strong>PostgreSQL:</strong> {{ statusData.db_postgres }}</p>
    <p><strong>Redis:</strong> {{ statusData.db_redis }}</p>
    <p><strong>API:</strong> {{ apiResponseTime }}</p>
    <p><strong>UI:</strong> v{{ appVersion }} - <strong>API:</strong> v{{ statusData.backend_version }}</p>
  </div>
</template>
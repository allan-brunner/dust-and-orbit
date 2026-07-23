<template>
  <div class="login-container">
    <h2>Login / Sign Up</h2>

    <form @submit.prevent="handleRequestOtp" v-if="!stepTwo">
      <div>
        <label>Email Address</label>
        <input 
          type="email" 
          v-model="email" 
          placeholder="hello@example.com" 
          required 
        />
      </div>
      <button type="submit" :disabled="loading">
        {{ loading ? 'Sending...' : 'Send Magic Code' }}
      </button>
    </form>

    <form @submit.prevent="handleVerifyOtp" v-else>
      <p>We sent a 6-digit code to <strong>{{ email }}</strong></p>
      <div>
        <label>Enter Code</label>
        <input 
          type="text" 
          v-model="code" 
          placeholder="123456" 
          maxlength="6"
          required 
        />
      </div>
      <button type="submit" :disabled="loading">
        {{ loading ? 'Verifying...' : 'Login' }}
      </button>
      <button type="button" @click="stepTwo = false" class="text-btn">
        Wrong email?
      </button>
    </form>

    <div v-if="authStore.isAuthenticated" class="success-box">
      <p>✅ Successfully Authenticated!</p>
      <p><strong>Your User ID:</strong> {{ authStore.userId }}</p>
      <button @click="authStore.logout()">Logout</button>
    </div>

    <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useAuthStore } from '../stores/auth';

const authStore = useAuthStore();

const email = ref('');
const code = ref('');
const stepTwo = ref(false);
const loading = ref(false);
const errorMessage = ref('');

const handleRequestOtp = async () => {
  loading.value = true;
  errorMessage.value = '';
  try {
    await authStore.requestOtp(email.value);
    stepTwo.value = true;
  } catch (error: any) {
    errorMessage.value = error.response?.data || "Failed to send code.";
  } finally {
    loading.value = false;
  }
};

const handleVerifyOtp = async () => {
  loading.value = true;
  errorMessage.value = '';
  try {
    await authStore.verifyOtp(email.value, code.value);
  } catch (error: any) {
    errorMessage.value = error.response?.data || "Invalid code.";
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
.login-container { max-width: 400px; margin: 0 auto; padding: 20px; }
.error { color: red; margin-top: 10px; }
.success-box { background: #e6ffed; padding: 15px; border-radius: 8px; margin-top: 20px; color: black; }
.text-btn { background: none; border: none; color: gray; text-decoration: underline; cursor: pointer; margin-top: 10px;}
input { display: block; width: 100%; margin: 8px 0 16px; padding: 8px; }
button { padding: 10px 15px; cursor: pointer; }
</style>
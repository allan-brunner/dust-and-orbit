import { defineStore } from 'pinia';
import { ref } from 'vue';
import { api } from '../api';

export const useAuthStore = defineStore('auth', () => {
    const accessToken = ref<string | null>(null);
    const userId = ref<string | null>(null);
    const isAuthenticated = ref(false);

    const requestOtp = async (email: string) => {
        await api.post('/auth/request-otp', { email });
    };

    const verifyOtp = async (email: string, code: string) => {
        const response = await api.post('/auth/verify', { email, code });

        accessToken.value = response.data.access_token;

        isAuthenticated.value = true;

        api.defaults.headers.common['Authorization'] = `Bearer ${accessToken.value}`;
    };

    const initializeAuth = async () => {
        try {
            const response = await api.post('/auth/refresh');

            accessToken.value = response.data.access_token;

            isAuthenticated.value = true;

            api.defaults.headers.common['Authorization'] = `Bearer ${accessToken.value}`;
        } catch (error) {
            logout();
        }
    };

    const logout = async () => {
        try {
            await api.post('/auth/logout');
        } catch (error) {
            console.error("Server logout failed, clearing local state anyway", error);
        } finally {
            accessToken.value = null;
            isAuthenticated.value = false;
            delete api.defaults.headers.common['Authorization'];
        }
    };

    return {
        accessToken,
        userId,
        isAuthenticated,
        requestOtp,
        verifyOtp,
        initializeAuth,
        logout
    };
});
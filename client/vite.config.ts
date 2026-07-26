import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), tailwindcss()],

  server: {
    fs: {
      allow: ['..']
    },
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
        headers: {
          'X-Forwarded-For': '127.0.0.1'
        }
      },
      '/auth': {
        target: 'http://localhost:3000',
        changeOrigin: true,
        headers: {
          'X-Forwarded-For': '127.0.0.1'
        }
      },
      '/admin': {
        target: 'http://localhost:3000',
        changeOrigin: true,
        headers: {
          'X-Forwarded-For': '127.0.0.1'
        }
      },
    }
  }
})

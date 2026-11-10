import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: '127.0.0.1',
    watch: { ignored: ['**/src-tauri/**', '**/.tools/**'] },
  },
  optimizeDeps: { entries: ['src/main.tsx'] },
  envPrefix: ['VITE_', 'TAURI_ENV_*'],
  build: { target: 'chrome110' },
  test: {
    globals: true,
    clearMocks: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.test.{ts,tsx}'],
    restoreMocks: true,
  },
})

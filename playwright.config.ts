import { defineConfig } from '@playwright/test'

export default defineConfig({
  testDir: './tests/ui',
  fullyParallel: true,
  reporter: 'list',
  use: { baseURL: 'http://127.0.0.1:1420', browserName: 'chromium', channel: 'msedge' },
  projects: [
    { name: 'desktop', use: { viewport: { width: 1200, height: 800 } } },
    { name: 'compact', use: { viewport: { width: 390, height: 844 }, reducedMotion: 'reduce' } },
  ],
  // Release QA exercises the bundled frontend, avoiding dev optimizer reloads
  // during isolated browser contexts. Native IPC is tested separately.
  webServer: { command: 'npm run build && npm run preview -- --port 1420 --strictPort', url: 'http://127.0.0.1:1420', reuseExistingServer: false, timeout: 120000 },
})

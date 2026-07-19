import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  build: { assetsDir: '.' },
  esbuild: { drop: ['console', 'debugger'] },  // removes console.log() from code
  plugins: [svelte()],
  server: { host: '0.0.0.0', port: 8080 },
})

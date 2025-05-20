// @ts-check
import { defineConfig } from 'astro/config';

import svelte from '@astrojs/svelte';

// https://astro.build/config
export default defineConfig({
  server: {
    host: '0.0.0.0',
    allowedHosts: true
  },
  vite: {
    build: {
      rollupOptions: {
        output: {
          assetFileNames: 'assets/[hash][extname]',
          chunkFileNames: '[hash].js',
          entryFileNames: '[hash].js'
        }
      },
    },
  },
  integrations: [svelte()]
});
// @ts-check
import { defineConfig } from 'astro/config';

import svelte from '@astrojs/svelte';

// import tailwindcss from '@tailwindcss/vite';

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
          chunkFileNames: 'assets/[hash].js',
          entryFileNames: 'assets/[hash].js'
        }
      },
    },

    // plugins: [tailwindcss()]
  },
  integrations: [svelte()]
});
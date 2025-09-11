// @ts-check
import { defineConfig } from 'astro/config';

import tailwindcss from '@tailwindcss/vite';
import svelte from '@astrojs/svelte';
import pages from 'astro-pages';

import solidJs from '@astrojs/solid-js';

export default defineConfig({
  server: {
    host: '0.0.0.0',
    allowedHosts: true
  },
  vite: {
    define: {
      __APP_ENV__: {
        STORE_SUFFIX: JSON.stringify("__STORE_SUFFIX__")
      }
    },
    build: {
      rollupOptions: {
        output: {
          assetFileNames: 'assets/[hash][extname]',
          chunkFileNames: 'assets/[hash].js',
          entryFileNames: 'assets/[hash].js'
        }
      },
    },

    plugins: [tailwindcss()]
  },
  integrations: [
    pages('apps/web/pages'),
    svelte(),
    solidJs(),
  ]
});

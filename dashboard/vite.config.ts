import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from 'path';

export default defineConfig({
  plugins: [svelte()],
  server: {
    fs: {
      allow: [
        '..',
        path.resolve(__dirname, '../backend/bindings')
      ]
    },
    proxy: {
      '/api': {
        target: 'http://some.localhost:3000',
        changeOrigin: true,
      }
    }
  },
  resolve: {
    alias: {
      '@bindings': path.resolve(__dirname, '../backend/bindings'),
      '@components': path.resolve(__dirname, './src/components'),
      '@stores': path.resolve(__dirname, './src/stores'),
      '@pages': path.resolve(__dirname, './src/pages'),
      '@lib': path.resolve(__dirname, './src/lib'),
      '@': path.resolve(__dirname, './src')
    }
  },
});

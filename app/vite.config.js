import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import tsconfigPaths from 'vite-tsconfig-paths';
import postcss from './postcss.config.js'

export default defineConfig({
  plugins: [solidPlugin(), tsconfigPaths()],
  server: {
    host: true,
    port: 3000,
  },
  css: {
    postcss
  },
  build: {
    target: 'esnext',
  },
});

import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import tailwindcss from '@tailwindcss/vite';
import devtools from 'solid-devtools/vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [devtools(), solidPlugin(), tailwindcss()],
  base: './',
  server: {
    proxy: {
      "/api": "http://localhost:3000",
    },
  },
  build: {
    target: "esnext",
  },
});

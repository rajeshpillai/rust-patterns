import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import tailwind from '@tailwindcss/vite';

// When deploying to GitHub Pages as a project site, the site is served
// under /<repo-name>/. The deploy script sets GITHUB_PAGES=true.
// Local dev keeps the root base so the dev server works unchanged.
const base = process.env.GITHUB_PAGES === 'true' ? '/rust-patterns/' : '/';

export default defineConfig({
  base,
  plugins: [solid(), tailwind()],
  server: {
    port: 5173,
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    sourcemap: true,
  },
});

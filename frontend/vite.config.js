import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import tailwind from '@tailwindcss/vite';

// Set VITE_BASE="/your-repo-name/" for a project-site deploy, or leave
// unset for user/organization sites (served from the root).
const base = process.env.VITE_BASE ?? '/';

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

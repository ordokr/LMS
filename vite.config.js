import { defineConfig } from 'vite';
import leptosPlugin from 'vite-plugin-leptos';
import { visualizer } from 'rollup-plugin-visualizer';

export default defineConfig({
  plugins: [
    leptosPlugin(),
    visualizer({
      open: false,
      gzipSize: true,
      brotliSize: true,
    }),
  ],
  build: {
    target: 'esnext',
    minify: 'esbuild',
    rollupOptions: {
      output: {
        manualChunks: (id) => {
          // Split forum code into a separate chunk
          if (id.includes('forum/')) {
            return 'forum';
          }
          
          // Split syntax highlighting into its own chunk
          if (id.includes('highlight.js') || id.includes('syntax-highlight')) {
            return 'syntax';
          }
          
          // Split math rendering into its own chunk
          if (id.includes('katex') || id.includes('latex')) {
            return 'math';
          }
          
          // Main app chunk for core functionality
          if (id.includes('src/')) {
            return 'app';
          }
          
          // All other third-party dependencies
          return 'vendor';
        },
      },
    },
    cssCodeSplit: true,
    assetsInlineLimit: 4096,
  },
  server: {
    fs: {
      // Allow serving files from the project root
      allow: ['..'],
    },
  },
});
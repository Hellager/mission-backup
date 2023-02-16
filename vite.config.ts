import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
import { vueI18n } from '@intlify/vite-plugin-vue-i18n'
import { visualizer } from 'rollup-plugin-visualizer'
import Icons from 'unplugin-icons/vite'
import IconsResolver from 'unplugin-icons/resolver'
import { FileSystemIconLoader } from 'unplugin-icons/loaders'
import Markdown from 'vite-plugin-md'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    Markdown(),
    visualizer({
      emitFile: true,
      filename: 'report.html',
    }),
    AutoImport({
      resolvers: [ElementPlusResolver()],
    }),
    Components({
      resolvers: [
        ElementPlusResolver(),
        IconsResolver()],
    }),
    vueI18n({
      include: resolve(dirname(fileURLToPath(import.meta.url))),
    }),
    Icons({
      compiler: 'vue3',
      autoInstall: true,
      customCollections: {
        icons: FileSystemIconLoader('src/assets/svg', svg => svg.replace(/^<svg /, '<svg fill="currentColor" ')),
      },
    }),
  ],

  // Vite optons tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: ['es2021', 'chrome100', 'safari13'],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    rollupOptions: {
      output: {
        manualChunks: {
          echarts: ['echarts'],
        },
      },
    },
  },
})

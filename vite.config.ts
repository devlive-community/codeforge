import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

export default defineConfig(async () => ({
    plugins: [vue()],
    css: {
        postcss: './postcss.config.js'
    },
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: 'ws',
                host,
                port: 1421
            }
            : undefined,
        watch: {
            ignored: ['**/src-tauri/**']
        }
    }
}))

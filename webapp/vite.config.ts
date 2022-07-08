import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import * as path from "path"
import { reactRouterPlugin } from 'vite-plugin-next-react-router'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), reactRouterPlugin(),],
  resolve: { alias: { web3: path.resolve(__dirname, './node_modules/web3/dist/web3.min.js'), cluster: ''}, },
  build: {
    target: 'esnext',
    minify: false
  },
  define: {
    'process.env.NODE_DEBUG': 'false',
    'global': 'globalThis'
  }
})
// https://github.com/vitejs/vite/issues/2618
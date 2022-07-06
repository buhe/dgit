import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from "path"
import { reactRouterPlugin } from 'vite-plugin-next-react-router'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), reactRouterPlugin(),],
  define: {
    'process.env': process.env,
    'global': {}
  },
  resolve: { alias: { web3: path.resolve(__dirname, './node_modules/web3/dist/web3.min.js') }, }
})

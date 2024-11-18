import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  root: './',
  plugins: [react()],
  server: {
    host: true,
    port: 8080,
    open: true,
    proxy: {
      '/api': {
        target: 'http://flusso-api-gateway-api-gateway.default.svc.cluster.local:8081', // URL del backend interno
        changeOrigin: true,
      },
    },
  },
});

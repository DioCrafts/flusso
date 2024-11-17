import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';


export default defineConfig({
  root: './', // Adjust this path to point to where your `index.html` is located
  plugins: [react()],
  server: {
    host: true, // Permite conexiones externas
    port: 5173, // Cambia si hay conflictos de puertos
    open: true, // Abre el navegador automáticamente
    proxy: {
        '/api': 'http://localhost:3000', // Redirige las solicitudes al backend
      },
  },
});
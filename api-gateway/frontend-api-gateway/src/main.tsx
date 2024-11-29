// src/main.tsx
import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './index.css';
import { setupApiClient } from './services/api/axios-config';
import { MetricsCollector } from './services/interceptors/metrics';

// Inicializar servicios
setupApiClient();
MetricsCollector.getInstance();

// Renderizar la aplicación
const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}

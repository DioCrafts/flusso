// src/config/env.ts
interface Env {
  API_URL: string;
  NODE_ENV: 'development' | 'production' | 'test';
  APP_VERSION: string;
  DEBUG_MODE: boolean;
  API_TIMEOUT: number;
  WEBSOCKET_URL: string;
}

export const env: Env = {
  API_URL: import.meta.env.VITE_API_URL || 'http://localhost:3000',
  NODE_ENV: import.meta.env.MODE as 'development' | 'production' | 'test',
  APP_VERSION: import.meta.env.VITE_APP_VERSION || '1.0.0',
  DEBUG_MODE: import.meta.env.VITE_DEBUG_MODE === 'true',
  API_TIMEOUT: parseInt(import.meta.env.VITE_API_TIMEOUT || '30000'),
  WEBSOCKET_URL: import.meta.env.VITE_WEBSOCKET_URL || 'ws://localhost:3001'
};

// Funciones helper para el entorno
export const isDevelopment = () => env.NODE_ENV === 'development';
export const isProduction = () => env.NODE_ENV === 'production';
export const isTest = () => env.NODE_ENV === 'test';

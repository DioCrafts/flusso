// src/services/api/security-api.ts
import apiClient from './axios-config';

export const securityApi = {
  getSettings: () => 
    apiClient.get('/api/security/settings'),

  updateSettings: (settings: any) =>
    apiClient.put('/api/security/settings', settings),
};
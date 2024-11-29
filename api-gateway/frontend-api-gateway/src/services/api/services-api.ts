// src/services/api/services-api.ts
import apiClient from './axios-config';
import { Service } from '@/types/services';

export const servicesApi = {
  getAll: () => 
    apiClient.get<Service[]>('/services'),

  getStatus: (id: string) => 
    apiClient.get<Service>(`/services/${id}/status`),

  getMetrics: (id: string) => 
    apiClient.get(`/services/${id}/metrics`),

  updateConfig: (id: string, config: any) => 
    apiClient.put(`/services/${id}/config`, config)
};


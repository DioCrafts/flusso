// src/services/api/services-api.ts
import apiClient from './axios-config';

export const servicesApi = {
  getAllServices: () => 
    apiClient.get<Service[]>('/api/services'),

  getServiceDetails: (name: string) =>
    apiClient.get<ServiceDetails>(`/api/services/${name}`),

  getServiceMetrics: (name: string) =>
    apiClient.get<ServiceMetrics>(`/api/services/${name}/metrics`),
};

// Tipos
export interface Service {
  id: string;
  name: string;
  status: 'healthy' | 'warning' | 'error';
  endpoint: string;
  latency: number;
  uptime: string;
  requestsPerMinute: number;
  errorRate: string;
  lastChecked: string;
}

export interface ServiceMetrics {
  latency: number;
  requestsPerMinute: number;
  errorRate: number;
  uptime: number;
}
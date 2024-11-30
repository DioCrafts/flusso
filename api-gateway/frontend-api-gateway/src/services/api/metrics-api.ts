// src/services/api/metrics-api.ts
import apiClient from './axios-config';

export const metricsApi = {
  getGatewayMetrics: () => 
    apiClient.get('/api/metrics/gateway'),

  getServiceMetrics: (serviceName: string) =>
    apiClient.get(`/api/metrics/services/${serviceName}`),

  getRouteMetrics: (routeName: string) =>
    apiClient.get(`/api/metrics/routes/${routeName}`),
};


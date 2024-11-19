import axios from 'axios';

// Crear una instancia base de axios
const apiClient = axios.create({
  baseURL: '/api', // Base URL para todas las llamadas
});

// Backends
export const backendsApi = {
  listBackends: () => apiClient.get('/backends'),
  addBackend: (backend: any) => apiClient.post('/backends', backend),
  deleteBackend: (id: number) => apiClient.delete(`/backends/${id}`),
  healthCheck: () => apiClient.get('/health-check'),
};

// Gateways
export const gatewaysApi = {
  listGateways: () => apiClient.get('/gateways'),
  addGateway: (gateway: any) => apiClient.post('/gateways', gateway),
  deleteGateway: (id: number) => apiClient.delete(`/gateways/${id}`),
  configureTLS: (id: number, certificate: string) =>
    apiClient.post('/gateways/tls', { id, certificate }),
  gatewayMetrics: () => apiClient.get('/gateways/metrics'),
};

// Observability
export const observabilityApi = {
  getMetrics: () => apiClient.get('/observability/metrics'),
  getLogs: () => apiClient.get('/observability/logs'),
  getAlerts: () => apiClient.get('/observability/alerts'),
};

// Security
export const securityApi = {
  getPolicies: () => apiClient.get('/security/policies'),
  addPolicy: (policy: any) => apiClient.post('/security/policies', policy),
  deletePolicy: (name: string) => apiClient.delete(`/security/policies/${name}`),
  testProtectedEndpoint: (token: string) =>
    apiClient.get('/protected', {
      headers: { Authorization: `Bearer ${token}` },
    }),
};

// Exportar apiClient por si necesitas más personalización en el futuro
export default apiClient;
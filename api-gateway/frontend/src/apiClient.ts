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






import axios from 'axios';

const apiClient = axios.create({
  baseURL: '/api', // Vite redirigirá a http://backend-service:8081
});

export const listBackends = () => apiClient.get('/backends');
export const addBackend = (backend: any) => apiClient.post('/backends', backend);
export const deleteBackend = (id: number) => apiClient.delete(`/backends/${id}`);
export const healthCheck = () => apiClient.get('/health-check');





import axios from 'axios';

const apiClient = axios.create({
  baseURL: '/api',
});

export const listGateways = () => apiClient.get('/gateways');
export const addGateway = (gateway: any) => apiClient.post('/gateways', gateway);
export const deleteGateway = (id: number) => apiClient.delete(`/gateways/${id}`);
export const configureTLS = (id: number, certificate: string) =>
  apiClient.post('/gateways/tls', { id, certificate });
export const gatewayMetrics = () => apiClient.get('/gateways/metrics');






import axios from 'axios';

const apiClient = axios.create({
  baseURL: '/api',
});

export const getObservabilityMetrics = () => apiClient.get('/observability/metrics');
export const getObservabilityLogs = () => apiClient.get('/observability/logs');


import axios from 'axios';

const apiClient = axios.create({
  baseURL: '/api',
});

export const getSecurityPolicies = () => apiClient.get('/security/policies');
export const addSecurityPolicy = (policy: any) => apiClient.post('/security/policies', policy);
export const deleteSecurityPolicy = (name: string) =>
  apiClient.delete(`/security/policies/${name}`);
export const testProtectedEndpoint = (token: string) =>
  apiClient.get('/protected', {
    headers: { Authorization: `Bearer ${token}` },
  });

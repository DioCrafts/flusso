import axios from 'axios';

// Configura Axios para que apunte al API REST en Kubernetes
const apiClient = axios.create({
  baseURL: 'http://flusso-api-gateway-api-gateway.default.svc.cluster.local:8081/api', // URL del API REST
});

// Funciones para consumir la API REST
export const getMetrics = async () => {
  const response = await apiClient.get('/metrics');
  return response.data;
};

export const getAlerts = async () => {
  const response = await apiClient.get('/alerts');
  return response.data;
};

export const getBackends = async () => {
  const response = await apiClient.get('/backends');
  return response.data;
};

export const deleteBackend = async (id: number) => {
  const response = await apiClient.delete(`/backends/${id}`);
  return response.data;
};

export const addBackend = async (backend: Record<string, unknown>) => {
  const response = await apiClient.post('/backends', backend);
  return response.data;
};

export default apiClient;

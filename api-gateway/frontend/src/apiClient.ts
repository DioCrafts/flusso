import axios from 'axios';

const apiClient = axios.create({
  baseURL: '/api', // Redirige automáticamente al backend usando el proxy de Vite
});

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

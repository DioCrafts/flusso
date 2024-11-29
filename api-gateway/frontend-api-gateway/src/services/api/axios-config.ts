import axios from 'axios';

// Define la URL base directamente en el código
const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'http://api-gateway-backend:3000',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json'
  }
});

export const setupApiClient = () => {
  // Request interceptor
  apiClient.interceptors.request.use(
    (config) => {
      const token = localStorage.getItem('token');
      if (token) {
        config.headers.Authorization = `Bearer ${token}`;
      }
      return config;
    },
    (error) => Promise.reject(error)
  );

  // Response interceptor
  apiClient.interceptors.response.use(
    (response) => response,
    async (error) => {
      if (error.response?.status === 401 && !error.config?._retry) {
        error.config._retry = true;
        try {
          const response = await apiClient.post('/auth/refresh');
          const { token } = response.data;
          localStorage.setItem('token', token);

          // Reintentar la petición original
          error.config.headers.Authorization = `Bearer ${token}`;
          return apiClient(error.config);
        } catch (refreshError) {
          localStorage.removeItem('token');
          window.location.href = '/login';
          return Promise.reject(refreshError);
        }
      }
      return Promise.reject(error);
    }
  );

  // Logging interceptor (opcional)
  apiClient.interceptors.request.use((config) => {
    console.log(`[API Request] ${config.method?.toUpperCase()} ${config.url}`);
    return config;
  });

  apiClient.interceptors.response.use(
    (response) => {
      console.log(`[API Response] ${response.status} ${response.config.url}`);
      return response;
    },
    (error) => {
      console.error('[API Error]', {
        url: error.config?.url,
        status: error.response?.status,
        message: error.message,
      });
      return Promise.reject(error);
    }
  );
};

export default apiClient;

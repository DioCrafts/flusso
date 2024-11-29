// src/services/interceptors/index.ts
import { AxiosInstance, AxiosError, InternalAxiosRequestConfig } from 'axios';
import { authService } from '../auth/auth-service';

export const interceptors = {
  setupInterceptors: (axiosInstance: AxiosInstance) => {
    // Request interceptor
    axiosInstance.interceptors.request.use(
      (config: InternalAxiosRequestConfig) => {
        const token = localStorage.getItem('token');
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
      },
      (error: AxiosError) => {
        return Promise.reject(error);
      }
    );

    // Response interceptor
    axiosInstance.interceptors.response.use(
      (response) => response,
      async (error: AxiosError) => {
        const originalRequest = error.config;
        
        // Si el error es 401 y no es un retry
        if (error.response?.status === 401 && !originalRequest?.headers['X-Retry']) {
          try {
            // Intentar refresh token
            await authService.refreshToken();
            
            // Reintentar la petición original
            if (originalRequest) {
              originalRequest.headers['X-Retry'] = 'true';
              return axiosInstance(originalRequest);
            }
          } catch (refreshError) {
            // Si falla el refresh, logout
            authService.logout();
            window.location.href = '/login';
          }
        }
        
        return Promise.reject(error);
      }
    );

    // Logging interceptor
    axiosInstance.interceptors.request.use(
      (config) => {
        console.log(`[API Request] ${config.method?.toUpperCase()} ${config.url}`);
        return config;
      }
    );

    axiosInstance.interceptors.response.use(
      (response) => {
        console.log(`[API Response] ${response.status} ${response.config.url}`);
        return response;
      },
      (error) => {
        console.error('[API Error]', {
          url: error.config?.url,
          status: error.response?.status,
          message: error.message
        });
        return Promise.reject(error);
      }
    );
  }
};

// src/services/auth/auth-service.ts
import apiClient from '../api/axios-config';
import { User, LoginResponse } from '@/types/auth';

export interface LoginCredentials {
  email: string;
  password: string;
}

export const authService = {
  login: async (credentials: LoginCredentials) => {
    const response = await apiClient.post<LoginResponse>('/auth/login', credentials);
    if (response.data.token) {
      localStorage.setItem('token', response.data.token);
      apiClient.defaults.headers.common['Authorization'] = `Bearer ${response.data.token}`;
    }
    return response.data;
  },

  logout: () => {
    localStorage.removeItem('token');
    delete apiClient.defaults.headers.common['Authorization'];
  },

  getCurrentUser: () => 
    apiClient.get<User>('/auth/me'),

  refreshToken: async () => {
    const token = localStorage.getItem('refresh_token');
    if (!token) throw new Error('No refresh token');

    const response = await apiClient.post<LoginResponse>('/auth/refresh', { token });
    if (response.data.token) {
      localStorage.setItem('token', response.data.token);
      apiClient.defaults.headers.common['Authorization'] = `Bearer ${response.data.token}`;
    }
    return response.data;
  }
};



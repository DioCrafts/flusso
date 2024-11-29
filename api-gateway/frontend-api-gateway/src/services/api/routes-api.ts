// src/services/api/routes-api.ts
import apiClient from './axios-config';
import { Route } from '@/types/routes';

export const routesApi = {
  getAll: () => 
    apiClient.get<Route[]>('/routes'),

  getById: (id: string) => 
    apiClient.get<Route>(`/routes/${id}`),

  create: (route: Omit<Route, 'id'>) => 
    apiClient.post<Route>('/routes', route),

  update: (id: string, route: Partial<Route>) => 
    apiClient.put<Route>(`/routes/${id}`, route),

  delete: (id: string) => 
    apiClient.delete(`/routes/${id}`)
};

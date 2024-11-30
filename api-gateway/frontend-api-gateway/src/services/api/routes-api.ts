// src/services/api/routes-api.ts
import apiClient from './axios-config';
import { Route } from '@/types/routes';

// Actualizar la interfaz Route para coincidir con el GatewayRoute de Kubernetes
export interface KubernetesRoute extends Route {
  metadata: {
    name: string;
    namespace?: string;
  };
  spec: {
    path: string;
    targetService: string;
    method: string;
    authRequired: boolean;
  };
}

export const routesApi = {
  getAll: () => 
    apiClient.get<KubernetesRoute[]>('/api/gateway-routes'),

  getById: (id: string) =>
    apiClient.get<KubernetesRoute>(`/api/gateway-routes/${id}`),

  create: (route: Omit<KubernetesRoute['spec'], 'id'>) =>
    apiClient.post<KubernetesRoute>('/api/gateway-routes', {
      apiVersion: 'gateway.api.k8s.io/v1alpha1',
      kind: 'GatewayRoute',
      metadata: {
        name: route.path.replace(/[^a-z0-9-]/g, '-'), // Generar un nombre válido para k8s
      },
      spec: route
    }),

  update: (name: string, route: Partial<KubernetesRoute['spec']>) =>
    apiClient.put<KubernetesRoute>(`/api/gateway-routes/${name}`, {
      spec: route
    }),

  delete: (name: string) =>
    apiClient.delete(`/api/gateway-routes/${name}`),

  // Métodos adicionales específicos de Kubernetes
  getStatus: (name: string) =>
    apiClient.get<KubernetesRoute>(`/api/gateway-routes/${name}/status`),

  // Para obtener rutas por namespace
  getAllInNamespace: (namespace: string) =>
    apiClient.get<KubernetesRoute[]>(`/api/gateway-routes?namespace=${namespace}`),
};
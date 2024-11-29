// src/stores/services/index.ts
import { create } from 'zustand';
import { ServicesState, Service } from './types';

export const useServicesStore = create<ServicesState>((set, get) => ({
  services: [],
  isLoading: false,
  error: null,
  selectedService: null,

  fetchServices: async () => {
    set({ isLoading: true, error: null });
    try {
      const response = await fetch('/api/services');
      if (!response.ok) throw new Error('Failed to fetch services');
      const services = await response.json();
      set({ services, isLoading: false });
    } catch (error) {
      set({ error: error instanceof Error ? error.message : 'Unknown error', isLoading: false });
    }
  },

  updateServiceStatus: async (id, status) => {
    try {
      const response = await fetch(`/api/services/${id}/status`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status }),
      });
      if (!response.ok) throw new Error('Failed to update service status');
      get().fetchServices();
    } catch (error) {
      set({ error: error instanceof Error ? error.message : 'Unknown error' });
    }
  },

  getServiceMetrics: async (id) => {
    try {
      const response = await fetch(`/api/services/${id}/metrics`);
      if (!response.ok) throw new Error('Failed to fetch service metrics');
      const metrics = await response.json();
      
      set(state => ({
        services: state.services.map(service => 
          service.id === id ? { ...service, metrics } : service
        )
      }));
    } catch (error) {
      set({ error: error instanceof Error ? error.message : 'Unknown error' });
    }
  },

  setSelectedService: (service) => set({ selectedService: service }),
}));

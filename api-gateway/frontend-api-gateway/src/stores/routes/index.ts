// src/stores/routes/index.ts
import { create } from 'zustand';
import { RoutesState, Route } from './types';

export const useRoutesStore = create<RoutesState>((set, get) => ({
  routes: [],
  isLoading: false,
  error: null,
  selectedRoute: null,

  fetchRoutes: async () => {
    set({ isLoading: true, error: null });
    try {
      const response = await fetch('/api/routes');
      if (!response.ok) throw new Error('Failed to fetch routes');
      const routes = await response.json();
      set({ routes, isLoading: false });
    } catch (error) {
      set({ error: error instanceof Error ? error.message : 'Unknown error', isLoading: false });
    }
  },

  addRoute: async (route) => {
    set({ isLoading: true, error: null });
    try {
      const response = await fetch('/api/routes', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(route),
      });
      if (!response.ok) throw new Error('Failed to add route');
      get().fetchRoutes();
    } catch (error) {
      set({ error: error instanceof Error ? error.message : 'Unknown error', isLoading: false });
    }
  },

  updateRoute: async (id, route) => {
    set({ isLoading: true, error: null });
    try {
      const response = await fetch(`/api/routes/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(route),
      });
      if (!response.ok) throw new Error('Failed to update route');
      get().fetchRoutes();
    } catch (error) {
      set({ error: error instanceof Error ? error.message : 'Unknown error', isLoading: false });
    }
  },

  deleteRoute: async (id) => {
    set({ isLoading: true, error: null });
    try {
      const response = await fetch(`/api/routes/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) throw new Error('Failed to delete route');
      get().fetchRoutes();
    } catch (error) {
      set({ error: error instanceof Error ? error.message : 'Unknown error', isLoading: false });
    }
  },

  setSelectedRoute: (route) => set({ selectedRoute: route }),
}));

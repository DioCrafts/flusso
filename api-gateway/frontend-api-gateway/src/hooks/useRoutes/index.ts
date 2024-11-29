// src/hooks/useRoutes/index.ts
import { useState, useCallback } from 'react';
import { Route } from '@/types/routes';
import { UseRoutesReturn } from './types';

export function useRoutes(): UseRoutesReturn {
  const [routes, setRoutes] = useState<Route[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchRoutes = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await fetch('/api/routes');
      if (!response.ok) throw new Error('Failed to fetch routes');
      const data = await response.json();
      setRoutes(data);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error'));
    } finally {
      setIsLoading(false);
    }
  }, []);

  const createRoute = useCallback(async (route: Omit<Route, 'id'>) => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await fetch('/api/routes', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(route)
      });
      if (!response.ok) throw new Error('Failed to create route');
      await fetchRoutes();
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error'));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [fetchRoutes]);

  const updateRoute = useCallback(async (id: string, route: Partial<Route>) => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await fetch(`/api/routes/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(route)
      });
      if (!response.ok) throw new Error('Failed to update route');
      await fetchRoutes();
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error'));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [fetchRoutes]);

  const deleteRoute = useCallback(async (id: string) => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await fetch(`/api/routes/${id}`, {
        method: 'DELETE'
      });
      if (!response.ok) throw new Error('Failed to delete route');
      await fetchRoutes();
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error'));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [fetchRoutes]);

  return {
    routes,
    isLoading,
    error,
    fetchRoutes,
    createRoute,
    updateRoute,
    deleteRoute
  };
}

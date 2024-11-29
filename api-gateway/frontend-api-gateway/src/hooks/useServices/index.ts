// src/hooks/useServices/index.ts
import { useState, useCallback } from 'react';
import { Service } from '@/types/services';
import { UseServicesReturn } from './types';

export function useServices(): UseServicesReturn {
  const [services, setServices] = useState<Service[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchServices = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await fetch('/api/services');
      if (!response.ok) throw new Error('Failed to fetch services');
      const data = await response.json();
      setServices(data);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error'));
    } finally {
      setIsLoading(false);
    }
  }, []);

  const getServiceHealth = useCallback(async (id: string) => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await fetch(`/api/services/${id}/health`);
      if (!response.ok) throw new Error('Failed to get service health');
      const data = await response.json();
      setServices(prev => prev.map(service => 
        service.id === id ? { ...service, ...data } : service
      ));
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error'));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  const updateServiceConfig = useCallback(async (id: string, config: Partial<Service>) => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await fetch(`/api/services/${id}/config`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config)
      });
      if (!response.ok) throw new Error('Failed to update service config');
      await fetchServices();
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error'));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [fetchServices]);

  const monitorService = useCallback(async (id: string) => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await fetch(`/api/services/${id}/metrics`);
      if (!response.ok) throw new Error('Failed to get service metrics');
      const data = await response.json();
      return data;
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error'));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  return {
    services,
    isLoading,
    error,
    fetchServices,
    getServiceHealth,
    updateServiceConfig,
    monitorService
  };
}

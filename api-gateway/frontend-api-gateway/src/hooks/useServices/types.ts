// src/hooks/useServices/types.ts
import { Service } from '@/types/services';

export interface UseServicesReturn {
  services: Service[];
  isLoading: boolean;
  error: Error | null;
  fetchServices: () => Promise<void>;
  getServiceHealth: (id: string) => Promise<void>;
  updateServiceConfig: (id: string, config: Partial<Service>) => Promise<void>;
  monitorService: (id: string) => Promise<void>;
}

// src/stores/services/types.ts
export interface Service {
  id: string;
  name: string;
  url: string;
  status: 'healthy' | 'warning' | 'error';
  metrics: {
    requestsPerSecond: number;
    averageLatency: number;
    errorRate: number;
  };
}

export interface ServicesState {
  services: Service[];
  isLoading: boolean;
  error: string | null;
  selectedService: Service | null;
  fetchServices: () => Promise<void>;
  updateServiceStatus: (id: string, status: Service['status']) => Promise<void>;
  getServiceMetrics: (id: string) => Promise<void>;
  setSelectedService: (service: Service | null) => void;
}

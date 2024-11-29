// src/types/services.ts
export interface Service {
  id: string;
  name: string;
  url: string;
  type: ServiceType;
  status: ServiceStatus;
  version: string;
  metrics: ServiceMetrics;
  config: ServiceConfig;
  health: HealthStatus;
}

export type ServiceType = 'http' | 'grpc' | 'websocket';

export type ServiceStatus = 'active' | 'inactive' | 'maintenance';

export interface ServiceMetrics {
  requestsPerSecond: number;
  averageLatency: number;
  errorRate: number;
  uptimePercentage: number;
  lastUpdated: string;
  responseTimeHistogram: {
    bucket: number;
    count: number;
  }[];
}

export interface ServiceConfig {
  timeout: number;
  maxConcurrentRequests: number;
  circuitBreaker: {
    enabled: boolean;
    failureThreshold: number;
    resetTimeoutMs: number;
  };
  loadBalancing: {
    strategy: 'round-robin' | 'least-connections' | 'random';
    healthCheck: {
      enabled: boolean;
      intervalMs: number;
      path: string;
    };
  };
}

export interface HealthStatus {
  status: 'healthy' | 'warning' | 'error';
  lastCheck: string;
  details: {
    cpu: number;
    memory: number;
    disk: number;
  };
}

// src/types/routes.ts
export interface Route {
  id: string;
  path: string;
  targetService: string;
  method: HttpMethod;
  authRequired: boolean;
  rateLimit?: RateLimitConfig;
  timeout?: number;
  retryPolicy?: RetryPolicy;
  metadata: RouteMetadata;
}

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';

export interface RateLimitConfig {
  requestsPerSecond: number;
  burstSize: number;
  enabled: boolean;
}

export interface RetryPolicy {
  maxAttempts: number;
  backoffMs: number;
  enabled: boolean;
}

export interface RouteMetadata {
  createdAt: string;
  updatedAt: string;
  version: number;
  tags: string[];
}

export interface RouteStats {
  requestCount: number;
  errorCount: number;
  averageLatency: number;
  p95Latency: number;
  lastUpdated: string;
}


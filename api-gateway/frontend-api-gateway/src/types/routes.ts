// src/types/routes.ts
export interface GatewayRoute {
  apiVersion: 'gateway.api.k8s.io/v1alpha1';
  kind: 'GatewayRoute';
  metadata: K8sMetadata;
  spec: RouteSpec;
  status?: RouteStatus;
}

export interface K8sMetadata {
  name: string;
  namespace?: string;
  creationTimestamp?: string;
  labels?: Record<string, string>;
  annotations?: Record<string, string>;
  generation?: number;
}

export interface RouteSpec {
  path: string;
  targetService: {
    name: string;
    namespace?: string;
    port?: number;
  };
  method: HttpMethod;
  rules: {
    auth?: AuthConfig;
    rateLimit?: RateLimitConfig;
    timeout?: number;
    retry?: RetryPolicy;
  };
}

export interface AuthConfig {
  required: boolean;
  scopes?: string[];
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

export interface RouteStatus {
  observedGeneration?: number;
  conditions: RouteCondition[];
  metrics?: RouteMetrics;
}

export interface RouteCondition {
  type: 'Ready' | 'Valid' | 'Accepted';
  status: 'True' | 'False' | 'Unknown';
  lastTransitionTime: string;
  reason: string;
  message: string;
}

export interface RouteMetrics {
  requestCount: number;
  errorCount: number;
  averageLatency: number;
  p95Latency: number;
  lastUpdated: string;
}
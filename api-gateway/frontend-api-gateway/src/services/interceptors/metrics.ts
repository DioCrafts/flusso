// src/services/interceptors/metrics.ts
import { AxiosInstance } from 'axios';

interface RequestMetrics {
  timestamp: number;
  duration: number;
  status: number;
  url: string;
  method: string;
}

export class MetricsCollector {
  private static instance: MetricsCollector;
  private metrics: RequestMetrics[] = [];

  private constructor() {}

  static getInstance(): MetricsCollector {
    if (!MetricsCollector.instance) {
      MetricsCollector.instance = new MetricsCollector();
    }
    return MetricsCollector.instance;
  }

  setupMetricsInterceptor(axiosInstance: AxiosInstance) {
    axiosInstance.interceptors.request.use((config) => {
      config.metadata = { startTime: Date.now() };
      return config;
    });

    axiosInstance.interceptors.response.use(
      (response) => {
        this.collectMetrics(response);
        return response;
      },
      (error) => {
        if (error.response) {
          this.collectMetrics(error.response);
        }
        return Promise.reject(error);
      }
    );
  }

  private collectMetrics(response: any) {
    const startTime = response.config.metadata.startTime;
    const endTime = Date.now();
    const duration = endTime - startTime;

    const metric: RequestMetrics = {
      timestamp: startTime,
      duration,
      status: response.status,
      url: response.config.url || '',
      method: response.config.method || '',
    };

    this.metrics.push(metric);
    this.pruneOldMetrics();
  }

  private pruneOldMetrics() {
    const oneHourAgo = Date.now() - 3600000;
    this.metrics = this.metrics.filter(m => m.timestamp > oneHourAgo);
  }

  getMetrics() {
    return this.metrics;
  }

  getAverageResponseTime() {
    if (this.metrics.length === 0) return 0;
    const sum = this.metrics.reduce((acc, m) => acc + m.duration, 0);
    return sum / this.metrics.length;
  }

  getErrorRate() {
    if (this.metrics.length === 0) return 0;
    const errors = this.metrics.filter(m => m.status >= 400).length;
    return (errors / this.metrics.length) * 100;
  }
}

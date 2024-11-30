// src/pages/dashboard/index.tsx
import React, { useEffect, useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { MetricCard } from '@/components/features/monitoring/MetricCard';
import { Activity, Users, Server } from 'lucide-react';
import { metricsApi } from '@/services/api/metrics-api';
import { servicesApi } from '@/services/api/services-api';

// Interfaces para los datos
interface MetricsData {
  totalRequests: number;
  requestsChange: number;
  activeServices: number;
  servicesChange: number;
  trafficData: Array<{
    time: string;
    value: number;
  }>;
}

interface ServiceHealth {
  name: string;
  status: 'Healthy' | 'Warning' | 'Error';
  lastCheck: string;
}

export function DashboardPage() {
  const [metrics, setMetrics] = useState<MetricsData | null>(null);
  const [services, setServices] = useState<ServiceHealth[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    fetchDashboardData();
    const interval = setInterval(fetchDashboardData, 30000); // Actualizar cada 30s
    return () => clearInterval(interval);
  }, []);

  const fetchDashboardData = async () => {
    try {
      setIsLoading(true);
      // Obtener métricas de Kubernetes
      const [metricsResponse, servicesResponse] = await Promise.all([
        metricsApi.getGatewayMetrics(),
        servicesApi.getAllServicesHealth()
      ]);

      setMetrics(metricsResponse.data);
      setServices(servicesResponse.data);
    } catch (error) {
      console.error('Error fetching dashboard data:', error);
    } finally {
      setIsLoading(false);
    }
  };

  if (isLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="animate-spin h-8 w-8 border-4 border-primary border-t-transparent rounded-full" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Dashboard</h1>
      
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <MetricCard
          title="Total Requests"
          value={metrics?.totalRequests.toLocaleString() || '0'}
          change={`${metrics?.requestsChange >= 0 ? '+' : ''}${metrics?.requestsChange.toFixed(1)}%`}
          icon={<Activity className="h-4 w-4" />}
        />
        <MetricCard
          title="Active Services"
          value={metrics?.activeServices.toString() || '0'}
          change={`${metrics?.servicesChange >= 0 ? '+' : ''}${metrics?.servicesChange}`}
          icon={<Server className="h-4 w-4" />}
        />
        <MetricCard
          title="Gateway Routes"
          value={metrics?.totalRoutes.toString() || '0'}
          change={`${metrics?.routesChange >= 0 ? '+' : ''}${metrics?.routesChange}`}
          icon={<Users className="h-4 w-4" />}
        />
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Traffic Overview</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={metrics?.trafficData || []}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis dataKey="time" />
                  <YAxis />
                  <Tooltip />
                  <Line 
                    type="monotone" 
                    dataKey="value" 
                    stroke="#8884d8" 
                    strokeWidth={2} 
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>System Health</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {services.map((service) => (
                <div 
                  key={service.name}
                  className="flex items-center justify-between p-2 bg-background rounded"
                >
                  <span>{service.name}</span>
                  <span className={`
                    ${service.status === 'Healthy' ? 'text-green-500' : ''}
                    ${service.status === 'Warning' ? 'text-yellow-500' : ''}
                    ${service.status === 'Error' ? 'text-red-500' : ''}
                  `}>
                    {service.status}
                  </span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

export default DashboardPage;
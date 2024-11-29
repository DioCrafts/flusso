// src/pages/services/index.tsx
import React, { useState } from 'react';
import { 
  Card, 
  CardContent, 
  CardHeader, 
  CardTitle 
} from '@/components/ui/card';
import { CheckCircle, AlertTriangle, XCircle } from 'lucide-react';

// Datos simulados
const MOCK_SERVICES = [
  {
    id: '1',
    name: 'User Service',
    status: 'healthy',
    endpoint: '/api/users',
    latency: 45,
    uptime: '99.9%',
    requestsPerMinute: 150,
    errorRate: '0.01%',
    lastChecked: new Date().toISOString()
  },
  {
    id: '2',
    name: 'Authentication Service',
    status: 'warning',
    endpoint: '/api/auth',
    latency: 120,
    uptime: '98.5%',
    requestsPerMinute: 200,
    errorRate: '1.5%',
    lastChecked: new Date().toISOString()
  },
  {
    id: '3',
    name: 'Payment Service',
    status: 'healthy',
    endpoint: '/api/payments',
    latency: 65,
    uptime: '99.7%',
    requestsPerMinute: 80,
    errorRate: '0.05%',
    lastChecked: new Date().toISOString()
  },
  {
    id: '4',
    name: 'Order Service',
    status: 'error',
    endpoint: '/api/orders',
    latency: 350,
    uptime: '95.0%',
    requestsPerMinute: 50,
    errorRate: '5.0%',
    lastChecked: new Date().toISOString()
  }
];

export function ServicesPage() {
  const [services] = useState(MOCK_SERVICES);

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'healthy':
        return <CheckCircle className="h-5 w-5 text-green-500" />;
      case 'warning':
        return <AlertTriangle className="h-5 w-5 text-yellow-500" />;
      case 'error':
        return <XCircle className="h-5 w-5 text-red-500" />;
      default:
        return null;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'healthy':
        return 'text-green-500';
      case 'warning':
        return 'text-yellow-500';
      case 'error':
        return 'text-red-500';
      default:
        return '';
    }
  };

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Services</h1>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {services.map((service) => (
          <Card key={service.id}>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-lg font-medium">
                {service.name}
              </CardTitle>
              {getStatusIcon(service.status)}
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                <div className="flex justify-between items-center">
                  <span className="text-sm text-muted-foreground">Status</span>
                  <span className={getStatusColor(service.status)}>
                    {service.status.charAt(0).toUpperCase() + service.status.slice(1)}
                  </span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-muted-foreground">Endpoint</span>
                  <span className="font-mono text-sm">{service.endpoint}</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-muted-foreground">Latency</span>
                  <span>{service.latency}ms</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-muted-foreground">Uptime</span>
                  <span>{service.uptime}</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-muted-foreground">Requests/min</span>
                  <span>{service.requestsPerMinute}</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-muted-foreground">Error Rate</span>
                  <span>{service.errorRate}</span>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}

export default ServicesPage;
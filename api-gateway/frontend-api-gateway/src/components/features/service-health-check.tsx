// src/components/features/services/service-health-check.tsx
import React from 'react';
import { Card, CardHeader, CardContent, CardTitle } from '../../ui/card';
import { CheckCircle, AlertTriangle, XCircle } from 'lucide-react';

interface ServiceHealth {
  name: string;
  status: 'healthy' | 'warning' | 'error';
  latency: number;
}

interface ServiceHealthCheckProps {
  services: ServiceHealth[];
}

export function ServiceHealthCheck({ services }: ServiceHealthCheckProps) {
  const getStatusIcon = (status: ServiceHealth['status']) => {
    switch (status) {
      case 'healthy': return <CheckCircle className="text-green-500 h-5 w-5" />;
      case 'warning': return <AlertTriangle className="text-yellow-500 h-5 w-5" />;
      case 'error': return <XCircle className="text-red-500 h-5 w-5" />;
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Service Health Status</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {services.map(service => (
            <div 
              key={service.name}
              className="flex items-center justify-between p-2 bg-background rounded"
            >
              <div className="flex items-center gap-2">
                {getStatusIcon(service.status)}
                <span>{service.name}</span>
              </div>
              <span>{service.latency}ms</span>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}

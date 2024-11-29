// src/pages/dashboard/index.tsx
import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { MetricCard } from '@/components/features/monitoring/MetricCard';
import { Activity, Users, Server } from 'lucide-react';

const trafficData = [
  { time: '00:00', value: 100 },
  { time: '04:00', value: 300 },
  { time: '08:00', value: 600 },
  { time: '12:00', value: 800 },
  { time: '16:00', value: 500 },
  { time: '20:00', value: 200 },
];

export function DashboardPage() {
  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Dashboard</h1>
      
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <MetricCard
          title="Total Requests"
          value="1.2M"
          change="+12.3%"
          icon={<Activity className="h-4 w-4" />}
        />
        <MetricCard
          title="Active Users"
          value="8,521"
          change="+5.2%"
          icon={<Users className="h-4 w-4" />}
        />
        <MetricCard
          title="Active Services"
          value="12"
          change="+1"
          icon={<Server className="h-4 w-4" />}
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
                <LineChart data={trafficData}>
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
              {/* System health status items */}
              <div className="flex items-center justify-between p-2 bg-background rounded">
                <span>API Gateway</span>
                <span className="text-green-500">Healthy</span>
              </div>
              <div className="flex items-center justify-between p-2 bg-background rounded">
                <span>Auth Service</span>
                <span className="text-green-500">Healthy</span>
              </div>
              <div className="flex items-center justify-between p-2 bg-background rounded">
                <span>User Service</span>
                <span className="text-yellow-500">Warning</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

export default DashboardPage;

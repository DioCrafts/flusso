import React, { useEffect, useState } from 'react';
import { Line } from 'react-chartjs-2';
import {
  Table,
  TableHead,
  TableRow,
  TableCell,
  TableBody,
  Typography,
  Grid,
  Card,
  CardContent,
  LinearProgress,
} from '@mui/material';
import { observabilityApi } from '../apiClient';

const { getMetrics: getObservabilityMetrics, getLogs: getObservabilityLogs } = observabilityApi;

interface Metric {
  name: string;
  value: number;
  labels: Record<string, string>;
}

interface LogEntry {
  timestamp: string;
  client_ip: string;
  http_code: number;
  route: string;
  backend: string;
  latency_ms: number;
}

const Observability: React.FC = () => {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [metricsResponse, logsResponse] = await Promise.all([
          getObservabilityMetrics(),
          getObservabilityLogs(),
        ]);

        const validMetrics = metricsResponse.data.map((metric: any) => ({
          ...metric,
          labels: metric.labels || {}, // Normaliza labels
        }));

        setMetrics(validMetrics);
        setLogs(logsResponse.data);
      } catch (error) {
        console.error('Error fetching observability data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) {
    return <LinearProgress />;
  }

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h4">Observability</Typography>
      </Grid>

      {/* Métricas */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Typography variant="h6">Metrics</Typography>
            {metrics.length === 0 ? (
              <Typography>No metrics available.</Typography>
            ) : (
              <Line
                data={{
                  labels: metrics.map((metric) => metric.labels?.route || 'Unknown'),
                  datasets: [
                    {
                      label: 'Requests per Second',
                      data: metrics.map((metric) => metric.value),
                      borderColor: 'rgba(75,192,192,1)',
                      backgroundColor: 'rgba(75,192,192,0.2)',
                    },
                  ],
                }}
              />
            )}
          </CardContent>
        </Card>
      </Grid>

      {/* Logs */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Typography variant="h6">Logs</Typography>
            {logs.length === 0 ? (
              <Typography>No logs available.</Typography>
            ) : (
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Timestamp</TableCell>
                    <TableCell>Client IP</TableCell>
                    <TableCell>HTTP Code</TableCell>
                    <TableCell>Route</TableCell>
                    <TableCell>Backend</TableCell>
                    <TableCell>Latency (ms)</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {logs.map((log, index) => (
                    <TableRow key={index}>
                      <TableCell>{new Date(log.timestamp).toLocaleString()}</TableCell>
                      <TableCell>{log.client_ip}</TableCell>
                      <TableCell>{log.http_code}</TableCell>
                      <TableCell>{log.route}</TableCell>
                      <TableCell>{log.backend}</TableCell>
                      <TableCell>{log.latency_ms}</TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            )}
          </CardContent>
        </Card>
      </Grid>
    </Grid>
  );
};

export default Observability;

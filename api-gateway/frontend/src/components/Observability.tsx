import React, { useEffect, useState } from 'react';
import {
  Grid,
  Card,
  CardContent,
  Typography,
  Table,
  TableHead,
  TableRow,
  TableCell,
  TableBody,
  LinearProgress,
} from '@mui/material';
import { Line, Bar, Pie } from 'react-chartjs-2';
import axios from 'axios';

// Define interfaces for observability metrics
interface Metric {
  timestamp: string;
  rps: number;
  latency: number;
  errors: number;
}

const Observability: React.FC = () => {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [loading, setLoading] = useState(true);

  // Fetch observability data
  const fetchMetrics = async () => {
    try {
      const response = await axios.get('/api/metrics'); // Replace with your API endpoint
      setMetrics(response.data);
      setLoading(false);
    } catch (error) {
      console.error('Error fetching metrics:', error);
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchMetrics();
  }, []);

  if (loading) {
    return <LinearProgress />;
  }

  // Process data for charts
  const timestamps = metrics.map((metric) => metric.timestamp);
  const rpsData = metrics.map((metric) => metric.rps);
  const latencyData = metrics.map((metric) => metric.latency);
  const errorData = metrics.map((metric) => metric.errors);

  const lineChartData = {
    labels: timestamps,
    datasets: [
      {
        label: 'Requests Per Second (RPS)',
        data: rpsData,
        borderColor: 'rgba(75, 192, 192, 1)',
        backgroundColor: 'rgba(75, 192, 192, 0.2)',
        tension: 0.4,
      },
    ],
  };

  const barChartData = {
    labels: timestamps,
    datasets: [
      {
        label: 'Latency (ms)',
        data: latencyData,
        backgroundColor: 'rgba(54, 162, 235, 0.7)',
      },
    ],
  };

  const pieChartData = {
    labels: ['2xx', '4xx', '5xx'],
    datasets: [
      {
        label: 'Error Distribution',
        data: [
          metrics.reduce((acc, metric) => acc + (metric.errors > 0 ? metric.errors * 0.5 : 0), 0), // Mock data
          metrics.reduce((acc, metric) => acc + (metric.errors > 0 ? metric.errors * 0.3 : 0), 0),
          metrics.reduce((acc, metric) => acc + (metric.errors > 0 ? metric.errors * 0.2 : 0), 0),
        ],
        backgroundColor: ['#4caf50', '#ff9800', '#f44336'],
      },
    ],
  };

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h4" gutterBottom>
          Observability Dashboard
        </Typography>
      </Grid>

      {/* Line Chart for RPS */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Requests Per Second (RPS) Over Time
            </Typography>
            <Line data={lineChartData} />
          </CardContent>
        </Card>
      </Grid>

      {/* Bar Chart for Latency */}
      <Grid item xs={12} md={6}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Latency Over Time (ms)
            </Typography>
            <Bar data={barChartData} />
          </CardContent>
        </Card>
      </Grid>

      {/* Pie Chart for Error Distribution */}
      <Grid item xs={12} md={6}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Error Distribution
            </Typography>
            <Pie data={pieChartData} />
          </CardContent>
        </Card>
      </Grid>

      {/* Raw Metrics Table */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Raw Metrics
            </Typography>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Timestamp</TableCell>
                  <TableCell>Requests Per Second (RPS)</TableCell>
                  <TableCell>Latency (ms)</TableCell>
                  <TableCell>Errors</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {metrics.map((metric, index) => (
                  <TableRow key={index}>
                    <TableCell>{metric.timestamp}</TableCell>
                    <TableCell>{metric.rps}</TableCell>
                    <TableCell>{metric.latency}</TableCell>
                    <TableCell>{metric.errors}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </Grid>
    </Grid>
  );
};

export default Observability;

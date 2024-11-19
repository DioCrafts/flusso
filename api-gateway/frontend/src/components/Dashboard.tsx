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
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { observabilityApi } from '../apiClient'; // Importa el cliente API

// Registrar componentes y escalas de Chart.js
ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

// Define interfaces para tipos
interface Metric {
  name: string;
  value: string | number;
}

interface Alert {
  id: number;
  message: string;
  timestamp: string;
}

// Mock Chart Data (Puedes reemplazarlo con datos reales si es necesario)
const generateChartData = () => ({
  labels: ['1 min', '5 min', '10 min', '15 min', '30 min'],
  datasets: [
    {
      label: 'Requests Per Second',
      data: [120, 200, 150, 180, 220],
      borderColor: 'rgba(75,192,192,1)',
      backgroundColor: 'rgba(75,192,192,0.2)',
    },
  ],
});

// Opciones del gráfico
const chartOptions = {
  responsive: true,
  plugins: {
    legend: {
      position: 'top',
    },
    title: {
      display: true,
      text: 'Traffic Over Time',
    },
  },
  scales: {
    x: {
      type: 'category', // Escala categórica en el eje X
    },
    y: {
      beginAtZero: true, // Empieza en 0 en el eje Y
    },
  },
};

const Dashboard: React.FC = () => {
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Fetch data from API
  const fetchDashboardData = async () => {
    try {
      const [metricsResponse, alertsResponse] = await Promise.all([
        observabilityApi.getMetrics(),
        observabilityApi.getAlerts(),
      ]);
      setMetrics(metricsResponse.data); // Ajusta según la estructura de los datos
      setAlerts(alertsResponse.data); // Ajusta según la estructura de los datos
    } catch (err) {
      console.error('Error fetching dashboard data:', err);
      setError('Failed to load dashboard data.');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDashboardData();
  }, []);

  if (loading) {
    return <LinearProgress />;
  }

  if (error) {
    return (
      <div>
        <Typography variant="h6" color="error">
          {error}
        </Typography>
      </div>
    );
  }

  return (
    <Grid container spacing={3}>
      {/* Title Section */}
      <Grid item xs={12}>
        <Typography variant="h4" gutterBottom>
          API Gateway Dashboard
        </Typography>
      </Grid>

      {/* Metrics Section */}
      {metrics.map((metric, index) => (
        <Grid item xs={12} md={6} key={index}>
          <Card>
            <CardContent>
              <Typography variant="h5">{metric.name}</Typography>
              <Typography variant="h4">{metric.value}</Typography>
            </CardContent>
          </Card>
        </Grid>
      ))}

      {/* Alerts Section */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Typography variant="h5" gutterBottom>
              Recent Alerts
            </Typography>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Alert Message</TableCell>
                  <TableCell>Timestamp</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {alerts.length > 0 ? (
                  alerts.map((alert) => (
                    <TableRow key={alert.id}>
                      <TableCell>{alert.message}</TableCell>
                      <TableCell>
                        {new Date(alert.timestamp).toLocaleString()}
                      </TableCell>
                    </TableRow>
                  ))
                ) : (
                  <TableRow>
                    <TableCell colSpan={2} align="center">
                      No recent alerts.
                    </TableCell>
                  </TableRow>
                )}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </Grid>

      {/* Chart Section */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Typography variant="h5" gutterBottom>
              Traffic Over Time
            </Typography>
            <Line data={generateChartData()} options={chartOptions} />
          </CardContent>
        </Card>
      </Grid>
    </Grid>
  );
};

export default Dashboard;

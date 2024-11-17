#!/bin/bash

# Nombre del directorio del proyecto
PROJECT_DIR="flusso-gui"

# Paso 1: Crear el directorio del proyecto
echo "Creando el proyecto $PROJECT_DIR..."
mkdir -p $PROJECT_DIR/src/components

# Paso 2: Inicializar el proyecto de React
cd $PROJECT_DIR
npx create-react-app . || { echo "Error al crear el proyecto de React."; exit 1; }

# Paso 3: Instalar dependencias necesarias
echo "Instalando dependencias..."
npm install axios react-router-dom || { echo "Error al instalar dependencias."; exit 1; }

# Paso 4: Crear la estructura de directorios
mkdir -p src/components
mkdir -p public

# Paso 5: Crear los archivos principales

# Crear index.html
cat > public/index.html <<EOF
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Flusso API Gateway</title>
  </head>
  <body>
    <div id="root"></div>
  </body>
</html>
EOF

# Crear App.js
cat > src/App.js <<EOF
import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Dashboard from './components/Dashboard';
import GatewayForm from './components/GatewayForm';
import GatewayList from './components/GatewayList';

const App = () => {
  return (
    <Router>
      <div>
        <h1>Flusso API Gateway GUI</h1>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/create-gateway" element={<GatewayForm />} />
          <Route path="/gateways" element={<GatewayList />} />
        </Routes>
      </div>
    </Router>
  );
};

export default App;
EOF

# Crear api.js
cat > src/api.js <<EOF
import axios from 'axios';

// Configura la URL base de la API de Flusso
const API_URL = 'http://localhost:8080/api'; // Cambia esto a la URL de tu API

// Crear un cliente de Axios para realizar las solicitudes
const apiClient = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Funciones para interactuar con la API
export const fetchGateways = async () => {
  try {
    const response = await apiClient.get('/gateways');
    return response.data;
  } catch (error) {
    console.error('Error fetching Gateways:', error);
    return [];
  }
};

export const createGateway = async (gatewayData) => {
  try {
    const response = await apiClient.post('/gateways', gatewayData);
    return response.data;
  } catch (error) {
    console.error('Error creating Gateway:', error);
    throw error;
  }
};

export const fetchRoutes = async () => {
  try {
    const response = await apiClient.get('/routes');
    return response.data;
  } catch (error) {
    console.error('Error fetching Routes:', error);
    return [];
  }
};

export const fetchMetrics = async () => {
  try {
    const response = await apiClient.get('/metrics');
    return response.data;
  } catch (error) {
    console.error('Error fetching Metrics:', error);
    return {};
  }
};
EOF

# Crear .env
cat > .env <<EOF
REACT_APP_API_URL=http://localhost:8080/api
EOF

# Crear los componentes en src/components

# Crear Dashboard.js
cat > src/components/Dashboard.js <<EOF
import React, { useEffect, useState } from 'react';
import { fetchGateways, fetchMetrics } from '../api';
import MetricCharts from './MetricCharts';

const Dashboard = () => {
  const [gateways, setGateways] = useState([]);
  const [metrics, setMetrics] = useState({});

  useEffect(() => {
    const loadData = async () => {
      const fetchedGateways = await fetchGateways();
      setGateways(fetchedGateways);

      const fetchedMetrics = await fetchMetrics();
      setMetrics(fetchedMetrics);
    };

    loadData();
  }, []);

  return (
    <div>
      <h1>Flusso API Gateway Dashboard</h1>
      <div>
        <h2>Gateways</h2>
        <ul>
          {gateways.map(gateway => (
            <li key={gateway.name}>{gateway.name} - {gateway.status}</li>
          ))}
        </ul>
      </div>

      <MetricCharts metrics={metrics} />
    </div>
  );
};

export default Dashboard;
EOF

# Crear MetricCharts.js
cat > src/components/MetricCharts.js <<EOF
import React from 'react';
import { Line } from 'react-chartjs-2';
import { Chart as ChartJS } from 'chart.js/auto';

const MetricCharts = ({ metrics }) => {
  const chartData = {
    labels: metrics?.timestamps || [],
    datasets: [
      {
        label: 'Requests',
        data: metrics?.requests || [],
        borderColor: 'rgba(75, 192, 192, 1)',
        fill: false,
      },
      {
        label: 'Latency',
        data: metrics?.latency || [],
        borderColor: 'rgba(153, 102, 255, 1)',
        fill: false,
      },
    ],
  };

  return (
    <div>
      <h2>API Gateway Metrics</h2>
      <Line data={chartData} />
    </div>
  );
};

export default MetricCharts;
EOF

# Crear GatewayForm.js
cat > src/components/GatewayForm.js <<EOF
import React, { useState } from 'react';
import { createGateway } from '../api';

const GatewayForm = () => {
  const [gatewayData, setGatewayData] = useState({
    name: '',
    gatewayClassName: '',
    listeners: [{ name: '', protocol: '', port: 8080 }],
  });

  const handleChange = (e) => {
    const { name, value } = e.target;
    setGatewayData(prevState => ({
      ...prevState,
      [name]: value,
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await createGateway(gatewayData);
      alert('Gateway created successfully');
    } catch (error) {
      alert('Failed to create Gateway');
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h2>Create a new Gateway</h2>
      <div>
        <label>Name</label>
        <input
          type="text"
          name="name"
          value={gatewayData.name}
          onChange={handleChange}
          required
        />
      </div>
      <div>
        <label>Gateway Class</label>
        <input
          type="text"
          name="gatewayClassName"
          value={gatewayData.gatewayClassName}
          onChange={handleChange}
          required
        />
      </div>
      <div>
        <label>Listener Name</label>
        <input
          type="text"
          name="listeners[0].name"
          value={gatewayData.listeners[0].name}
          onChange={handleChange}
        />
      </div>
      <div>
        <label>Protocol</label>
        <input
          type="text"
          name="listeners[0].protocol"
          value={gatewayData.listeners[0].protocol}
          onChange={handleChange}
        />
      </div>
      <div>
        <label>Port</label>
        <input
          type="number"
          name="listeners[0].port"
          value={gatewayData.listeners[0].port}
          onChange={handleChange}
        />
      </div>
      <button type="submit">Create Gateway</button>
    </form>
  );
};

export default GatewayForm;
EOF

# Crear GatewayList.js
cat > src/components/GatewayList.js <<EOF
import React, { useEffect, useState } from 'react';
import { fetchGateways } from '../api';

const GatewayList = () => {
  const [gateways, setGateways] = useState([]);

  useEffect(() => {
    const loadData = async () => {
      const fetchedGateways = await fetchGateways();
      setGateways(fetchedGateways);
    };

    loadData();
  }, []);

  return (
    <div>
      <h2>List of Gateways</h2>
      <ul>
        {gateways.map(gateway => (
          <li key={gateway.name}>
            {gateway.name} - {gateway.status}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default GatewayList;
EOF

# Crear .env
cat > .env <<EOF
REACT_APP_API_URL=http://localhost:8080/api
EOF

# Paso 6: Ejecutar el servidor de desarrollo de React
npm start

echo "Entorno de Flusso GUI creado y ejecutado exitosamente."


import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Dashboard from './components/Dashboard';
import Gateways from './components/Gateways';
import Security from './components/Security';
import Observability from './components/Observability';
import Backends from './components/Backends';
import './App.css'; // Opcional: agrega estilos globales o themes

const App: React.FC = () => {
  return (
    <Router>
      <div>
        {/* Encabezado opcional: un Navbar o Sidebar */}
        <header>
          <h1>API Gateway GUI</h1>
        </header>

        {/* Definición de rutas */}
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/gateways" element={<Gateways />} />
          <Route path="/security" element={<Security />} />
          <Route path="/observability" element={<Observability />} />
          <Route path="/backends" element={<Backends />} />
        </Routes>
      </div>
    </Router>
  );
};

export default App;

import React from 'react';
import { BrowserRouter as Router, Route, Routes, Link } from 'react-router-dom';
import { AppBar, Toolbar, Button, Typography } from '@mui/material';
import Dashboard from './components/Dashboard';
import Gateways from './components/Gateways';
import Security from './components/Security';
import Observability from './components/Observability';
import Backends from './components/Backends';

const App: React.FC = () => {
  return (
    <Router>
      <div>
        {/* Navbar */}
        <AppBar position="static">
          <Toolbar>
            <Typography variant="h6" sx={{ flexGrow: 1 }}>
              API Gateway GUI
            </Typography>
            <Button color="inherit" component={Link} to="/">
              Dashboard
            </Button>
            <Button color="inherit" component={Link} to="/backends">
              Backends
            </Button>
            <Button color="inherit" component={Link} to="/gateways">
              Gateways
            </Button>
            <Button color="inherit" component={Link} to="/observability">
              Observability
            </Button>
            <Button color="inherit" component={Link} to="/security">
              Security
            </Button>
          </Toolbar>
        </AppBar>

        {/* Rutas */}
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/backends" element={<Backends />} />
          <Route path="/gateways" element={<Gateways />} />
          <Route path="/observability" element={<Observability />} />
          <Route path="/security" element={<Security />} />
        </Routes>
      </div>
    </Router>
  );
};

export default App;

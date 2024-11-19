import React, { useEffect, useState } from 'react';
import {
  Table,
  TableHead,
  TableRow,
  TableCell,
  TableBody,
  Button,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
} from '@mui/material';
import { listGateways, addGateway, deleteGateway, configureTLS } from '../apiClient';

const Gateways: React.FC = () => {
  const [gateways, setGateways] = useState<any[]>([]);
  const [openDialog, setOpenDialog] = useState(false);
  const [newGateway, setNewGateway] = useState({ hostname: '', tls_enabled: false, routes: [] });
  const [tlsDialog, setTlsDialog] = useState(false);
  const [tlsConfig, setTlsConfig] = useState({ id: 0, certificate: '' });

  const fetchGateways = async () => {
    const response = await listGateways();
    setGateways(response.data);
  };

  const handleAddGateway = async () => {
    await addGateway(newGateway);
    setOpenDialog(false);
    fetchGateways();
  };

  const handleDeleteGateway = async (id: number) => {
    await deleteGateway(id);
    fetchGateways();
  };

  const handleConfigureTLS = async () => {
    await configureTLS(tlsConfig.id, tlsConfig.certificate);
    setTlsDialog(false);
    fetchGateways();
  };

  useEffect(() => {
    fetchGateways();
  }, []);

  return (
    <div>
      <h2>Gateways</h2>
      <Button variant="contained" onClick={() => setOpenDialog(true)}>
        Add Gateway
      </Button>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Hostname</TableCell>
            <TableCell>TLS</TableCell>
            <TableCell>Routes</TableCell>
            <TableCell>Actions</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {gateways.map((gateway) => (
            <TableRow key={gateway.id}>
              <TableCell>{gateway.hostname}</TableCell>
              <TableCell>{gateway.tls_enabled ? 'Enabled' : 'Disabled'}</TableCell>
              <TableCell>{gateway.routes.length} routes</TableCell>
              <TableCell>
                <Button variant="outlined" onClick={() => setTlsDialog(true)}>
                  Configure TLS
                </Button>
                <Button variant="contained" onClick={() => handleDeleteGateway(gateway.id)}>
                  Delete
                </Button>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      {/* Dialog para agregar Gateway */}
      <Dialog open={openDialog} onClose={() => setOpenDialog(false)}>
        <DialogTitle>Add Gateway</DialogTitle>
        <DialogContent>
          <TextField
            label="Hostname"
            fullWidth
            onChange={(e) => setNewGateway({ ...newGateway, hostname: e.target.value })}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenDialog(false)}>Cancel</Button>
          <Button onClick={handleAddGateway}>Add</Button>
        </DialogActions>
      </Dialog>

      {/* Dialog para configurar TLS */}
      <Dialog open={tlsDialog} onClose={() => setTlsDialog(false)}>
        <DialogTitle>Configure TLS</DialogTitle>
        <DialogContent>
          <TextField
            label="Certificate"
            fullWidth
            onChange={(e) => setTlsConfig({ ...tlsConfig, certificate: e.target.value })}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setTlsDialog(false)}>Cancel</Button>
          <Button onClick={handleConfigureTLS}>Save</Button>
        </DialogActions>
      </Dialog>
    </div>
  );
};

export default Gateways;

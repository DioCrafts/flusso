import React, { useEffect, useState } from 'react';
import {
  Grid,
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
import { backendsApi } from '../apiClient';

const { listBackends, addBackend, deleteBackend, healthCheck } = backendsApi;

const Backends: React.FC = () => {
  const [backends, setBackends] = useState<any[]>([]);
  const [openDialog, setOpenDialog] = useState(false);
  const [newBackend, setNewBackend] = useState({ name: '', address: '', port: 80 });

  const fetchBackends = async () => {
    const response = await listBackends();
    setBackends(response.data);
  };

  const handleHealthCheck = async () => {
    const response = await healthCheck();
    setBackends(response.data);
  };

  const handleAddBackend = async () => {
    await addBackend(newBackend);
    setOpenDialog(false);
    fetchBackends();
  };

  const handleDeleteBackend = async (id: number) => {
    await deleteBackend(id);
    fetchBackends();
  };

  useEffect(() => {
    fetchBackends();
  }, []);

  return (
    <div>
      <h2>Backends</h2>
      <Button variant="contained" onClick={() => setOpenDialog(true)}>
        Add Backend
      </Button>
      <Button variant="outlined" onClick={handleHealthCheck}>
        Run Health Check
      </Button>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Name</TableCell>
            <TableCell>Address</TableCell>
            <TableCell>Port</TableCell>
            <TableCell>Status</TableCell>
            <TableCell>Actions</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {backends.map((backend) => (
            <TableRow key={backend.id}>
              <TableCell>{backend.name}</TableCell>
              <TableCell>{backend.address}</TableCell>
              <TableCell>{backend.port}</TableCell>
              <TableCell>{backend.status}</TableCell>
              <TableCell>
                <Button variant="contained" onClick={() => handleDeleteBackend(backend.id)}>
                  Delete
                </Button>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      <Dialog open={openDialog} onClose={() => setOpenDialog(false)}>
        <DialogTitle>Add Backend</DialogTitle>
        <DialogContent>
          <TextField
            label="Name"
            fullWidth
            onChange={(e) => setNewBackend({ ...newBackend, name: e.target.value })}
          />
          <TextField
            label="Address"
            fullWidth
            onChange={(e) => setNewBackend({ ...newBackend, address: e.target.value })}
          />
          <TextField
            label="Port"
            fullWidth
            type="number"
            onChange={(e) => setNewBackend({ ...newBackend, port: Number(e.target.value) })}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenDialog(false)}>Cancel</Button>
          <Button onClick={handleAddBackend}>Add</Button>
        </DialogActions>
      </Dialog>
    </div>
  );
};

export default Backends;

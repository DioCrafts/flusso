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
  Button,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  LinearProgress,
} from '@mui/material';
import axios from 'axios';

// Define interfaces for backends
interface Backend {
  id: number;
  name: string;
  address: string;
  port: number;
  status: string; // Healthy or Unhealthy
}

const Backends: React.FC = () => {
  const [backends, setBackends] = useState<Backend[]>([]);
  const [loading, setLoading] = useState(true);
  const [openDialog, setOpenDialog] = useState(false);
  const [newBackend, setNewBackend] = useState({
    name: '',
    address: '',
    port: 80,
  });

  // Fetch backends from API
  const fetchBackends = async () => {
    try {
      const response = await axios.get('/api/backends'); // Replace with your API endpoint
      setBackends(response.data);
      setLoading(false);
    } catch (error) {
      console.error('Error fetching backends:', error);
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchBackends();
  }, []);

  // Handle dialog state
  const handleDialogOpen = () => {
    setOpenDialog(true);
  };

  const handleDialogClose = () => {
    setOpenDialog(false);
    setNewBackend({ name: '', address: '', port: 80 });
  };

  // Handle adding a new backend
  const handleAddBackend = async () => {
    try {
      const response = await axios.post('/api/backends', newBackend); // Replace with your API endpoint
      setBackends([...backends, response.data]);
      handleDialogClose();
    } catch (error) {
      console.error('Error adding backend:', error);
    }
  };

  // Handle deleting a backend
  const handleDeleteBackend = async (id: number) => {
    try {
      await axios.delete(`/api/backends/${id}`); // Replace with your API endpoint
      setBackends(backends.filter((backend) => backend.id !== id));
    } catch (error) {
      console.error('Error deleting backend:', error);
    }
  };

  if (loading) {
    return <LinearProgress />;
  }

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h4" gutterBottom>
          Backends
        </Typography>
      </Grid>

      {/* Table of Backends */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
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
                      <Button
                        variant="outlined"
                        color="error"
                        onClick={() => handleDeleteBackend(backend.id)}
                      >
                        Delete
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </Grid>

      {/* Add Backend Button */}
      <Grid item xs={12}>
        <Button variant="contained" color="primary" onClick={handleDialogOpen}>
          Add Backend
        </Button>
      </Grid>

      {/* Add Backend Dialog */}
      <Dialog open={openDialog} onClose={handleDialogClose}>
        <DialogTitle>Add New Backend</DialogTitle>
        <DialogContent>
          <TextField
            label="Name"
            fullWidth
            margin="normal"
            value={newBackend.name}
            onChange={(e) => setNewBackend({ ...newBackend, name: e.target.value })}
          />
          <TextField
            label="Address"
            fullWidth
            margin="normal"
            value={newBackend.address}
            onChange={(e) => setNewBackend({ ...newBackend, address: e.target.value })}
            helperText="Example: 192.168.1.10 or backend.example.com"
          />
          <TextField
            label="Port"
            fullWidth
            margin="normal"
            type="number"
            value={newBackend.port}
            onChange={(e) => setNewBackend({ ...newBackend, port: parseInt(e.target.value, 10) })}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDialogClose} color="secondary">
            Cancel
          </Button>
          <Button onClick={handleAddBackend} color="primary">
            Add
          </Button>
        </DialogActions>
      </Dialog>
    </Grid>
  );
};

export default Backends;

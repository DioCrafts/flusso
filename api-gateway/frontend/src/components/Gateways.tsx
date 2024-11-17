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

// Define types for gateways
interface Gateway {
  id: number;
  name: string;
  protocol: string;
  ip: string;
  status: string;
}

const Gateways: React.FC = () => {
  const [gateways, setGateways] = useState<Gateway[]>([]);
  const [loading, setLoading] = useState(true);
  const [openDialog, setOpenDialog] = useState(false);
  const [newGateway, setNewGateway] = useState({
    name: '',
    protocol: 'HTTP',
    ip: '',
  });

  // Fetch gateways from API
  const fetchGateways = async () => {
    try {
      const response = await axios.get('/api/gateways'); // Replace with your API endpoint
      setGateways(response.data);
      setLoading(false);
    } catch (error) {
      console.error('Error fetching gateways:', error);
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchGateways();
  }, []);

  // Handle dialog state
  const handleDialogOpen = () => {
    setOpenDialog(true);
  };

  const handleDialogClose = () => {
    setOpenDialog(false);
    setNewGateway({ name: '', protocol: 'HTTP', ip: '' });
  };

  // Handle gateway creation
  const handleAddGateway = async () => {
    try {
      const response = await axios.post('/api/gateways', newGateway); // Replace with your API endpoint
      setGateways([...gateways, response.data]);
      handleDialogClose();
    } catch (error) {
      console.error('Error adding gateway:', error);
    }
  };

  // Handle gateway deletion
  const handleDeleteGateway = async (id: number) => {
    try {
      await axios.delete(`/api/gateways/${id}`); // Replace with your API endpoint
      setGateways(gateways.filter((gateway) => gateway.id !== id));
    } catch (error) {
      console.error('Error deleting gateway:', error);
    }
  };

  if (loading) {
    return <LinearProgress />;
  }

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h4" gutterBottom>
          Gateways
        </Typography>
      </Grid>

      {/* Table of Gateways */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Protocol</TableCell>
                  <TableCell>IP</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {gateways.map((gateway) => (
                  <TableRow key={gateway.id}>
                    <TableCell>{gateway.name}</TableCell>
                    <TableCell>{gateway.protocol}</TableCell>
                    <TableCell>{gateway.ip}</TableCell>
                    <TableCell>{gateway.status}</TableCell>
                    <TableCell>
                      <Button
                        variant="outlined"
                        color="error"
                        onClick={() => handleDeleteGateway(gateway.id)}
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

      {/* Add Gateway Button */}
      <Grid item xs={12}>
        <Button variant="contained" color="primary" onClick={handleDialogOpen}>
          Add Gateway
        </Button>
      </Grid>

      {/* Add Gateway Dialog */}
      <Dialog open={openDialog} onClose={handleDialogClose}>
        <DialogTitle>Add New Gateway</DialogTitle>
        <DialogContent>
          <TextField
            label="Name"
            fullWidth
            margin="normal"
            value={newGateway.name}
            onChange={(e) => setNewGateway({ ...newGateway, name: e.target.value })}
          />
          <TextField
            label="Protocol"
            fullWidth
            margin="normal"
            value={newGateway.protocol}
            onChange={(e) => setNewGateway({ ...newGateway, protocol: e.target.value })}
            helperText="Enter HTTP, HTTPS, or TCP"
          />
          <TextField
            label="IP Address"
            fullWidth
            margin="normal"
            value={newGateway.ip}
            onChange={(e) => setNewGateway({ ...newGateway, ip: e.target.value })}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDialogClose} color="secondary">
            Cancel
          </Button>
          <Button onClick={handleAddGateway} color="primary">
            Add
          </Button>
        </DialogActions>
      </Dialog>
    </Grid>
  );
};

export default Gateways;

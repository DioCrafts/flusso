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
  Select,
  MenuItem,
  LinearProgress,
  Switch,
} from '@mui/material';
import axios from 'axios';

// Define interfaces for security policies
interface SecurityPolicy {
  id: number;
  name: string;
  type: string;
  status: boolean; // Active or disabled
  details: string; // Extra details, e.g., IPs, JWT provider
}

const Security: React.FC = () => {
  const [policies, setPolicies] = useState<SecurityPolicy[]>([]);
  const [loading, setLoading] = useState(true);
  const [openDialog, setOpenDialog] = useState(false);
  const [newPolicy, setNewPolicy] = useState({
    name: '',
    type: 'IP Blocking',
    details: '',
    status: true,
  });

  // Fetch policies from API
  const fetchPolicies = async () => {
    try {
      const response = await axios.get('/api/security'); // Replace with your API endpoint
      setPolicies(response.data);
      setLoading(false);
    } catch (error) {
      console.error('Error fetching policies:', error);
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPolicies();
  }, []);

  // Handle dialog state
  const handleDialogOpen = () => {
    setOpenDialog(true);
  };

  const handleDialogClose = () => {
    setOpenDialog(false);
    setNewPolicy({ name: '', type: 'IP Blocking', details: '', status: true });
  };

  // Handle adding a new policy
  const handleAddPolicy = async () => {
    try {
      const response = await axios.post('/api/security', newPolicy); // Replace with your API endpoint
      setPolicies([...policies, response.data]);
      handleDialogClose();
    } catch (error) {
      console.error('Error adding policy:', error);
    }
  };

  // Handle deleting a policy
  const handleDeletePolicy = async (id: number) => {
    try {
      await axios.delete(`/api/security/${id}`); // Replace with your API endpoint
      setPolicies(policies.filter((policy) => policy.id !== id));
    } catch (error) {
      console.error('Error deleting policy:', error);
    }
  };

  // Handle toggling policy status
  const handleTogglePolicy = async (id: number, currentStatus: boolean) => {
    try {
      await axios.patch(`/api/security/${id}`, { status: !currentStatus }); // Replace with your API endpoint
      setPolicies(
        policies.map((policy) =>
          policy.id === id ? { ...policy, status: !currentStatus } : policy
        )
      );
    } catch (error) {
      console.error('Error toggling policy status:', error);
    }
  };

  if (loading) {
    return <LinearProgress />;
  }

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h4" gutterBottom>
          Security Policies
        </Typography>
      </Grid>

      {/* Table of Policies */}
      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Type</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Details</TableCell>
                  <TableCell>Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {policies.map((policy) => (
                  <TableRow key={policy.id}>
                    <TableCell>{policy.name}</TableCell>
                    <TableCell>{policy.type}</TableCell>
                    <TableCell>
                      <Switch
                        checked={policy.status}
                        onChange={() => handleTogglePolicy(policy.id, policy.status)}
                      />
                    </TableCell>
                    <TableCell>{policy.details}</TableCell>
                    <TableCell>
                      <Button
                        variant="outlined"
                        color="error"
                        onClick={() => handleDeletePolicy(policy.id)}
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

      {/* Add Policy Button */}
      <Grid item xs={12}>
        <Button variant="contained" color="primary" onClick={handleDialogOpen}>
          Add Policy
        </Button>
      </Grid>

      {/* Add Policy Dialog */}
      <Dialog open={openDialog} onClose={handleDialogClose}>
        <DialogTitle>Add New Security Policy</DialogTitle>
        <DialogContent>
          <TextField
            label="Name"
            fullWidth
            margin="normal"
            value={newPolicy.name}
            onChange={(e) => setNewPolicy({ ...newPolicy, name: e.target.value })}
          />
          <Select
            label="Type"
            fullWidth
            margin="normal"
            value={newPolicy.type}
            onChange={(e) => setNewPolicy({ ...newPolicy, type: e.target.value })}
          >
            <MenuItem value="IP Blocking">IP Blocking</MenuItem>
            <MenuItem value="Rate Limiting">Rate Limiting</MenuItem>
            <MenuItem value="JWT Authentication">JWT Authentication</MenuItem>
          </Select>
          <TextField
            label="Details"
            fullWidth
            margin="normal"
            value={newPolicy.details}
            onChange={(e) => setNewPolicy({ ...newPolicy, details: e.target.value })}
            helperText="Example: IP ranges, JWT provider URL, or rate limits"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDialogClose} color="secondary">
            Cancel
          </Button>
          <Button onClick={handleAddPolicy} color="primary">
            Add
          </Button>
        </DialogActions>
      </Dialog>
    </Grid>
  );
};

export default Security;

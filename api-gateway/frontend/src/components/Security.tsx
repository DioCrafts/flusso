import React, { useState, useEffect } from 'react';
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
} from '@mui/material';
import { securityApi } from '../apiClient';

const { getPolicies: getSecurityPolicies, addPolicy: addSecurityPolicy, deletePolicy: deleteSecurityPolicy } = securityApi;

interface Policy {
  name: string;
  description: string;
  active: boolean;
  rules: string[];
}

const Security: React.FC = () => {
  const [policies, setPolicies] = useState<Policy[]>([]);
  const [newPolicy, setNewPolicy] = useState<Policy>({
    name: '',
    description: '',
    active: true,
    rules: [],
  });
  const [openDialog, setOpenDialog] = useState(false);

  useEffect(() => {
    const fetchPolicies = async () => {
      const response = await getSecurityPolicies();
      setPolicies(response.data);
    };
    fetchPolicies();
  }, []);

  const handleAddPolicy = async () => {
    await addSecurityPolicy(newPolicy);
    setPolicies([...policies, newPolicy]);
    setOpenDialog(false);
    setNewPolicy({ name: '', description: '', active: true, rules: [] });
  };

  const handleDeletePolicy = async (name: string) => {
    await deleteSecurityPolicy(name);
    setPolicies(policies.filter((policy) => policy.name !== name));
  };

  return (
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h4">Security</Typography>
      </Grid>

      <Grid item xs={12}>
        <Card>
          <CardContent>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Description</TableCell>
                  <TableCell>Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {policies.map((policy, index) => (
                  <TableRow key={index}>
                    <TableCell>{policy.name}</TableCell>
                    <TableCell>{policy.description}</TableCell>
                    <TableCell>
                      <Button
                        variant="outlined"
                        color="error"
                        onClick={() => handleDeletePolicy(policy.name)}
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

      <Grid item xs={12}>
        <Button variant="contained" color="primary" onClick={() => setOpenDialog(true)}>
          Add Policy
        </Button>
      </Grid>

      <Dialog open={openDialog} onClose={() => setOpenDialog(false)}>
        <DialogTitle>Add New Security Policy</DialogTitle>
        <DialogContent>
          <TextField
            label="Name"
            fullWidth
            margin="normal"
            value={newPolicy.name}
            onChange={(e) => setNewPolicy({ ...newPolicy, name: e.target.value })}
          />
          <TextField
            label="Description"
            fullWidth
            margin="normal"
            value={newPolicy.description}
            onChange={(e) => setNewPolicy({ ...newPolicy, description: e.target.value })}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenDialog(false)} color="secondary">
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

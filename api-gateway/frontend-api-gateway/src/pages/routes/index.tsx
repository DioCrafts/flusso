// src/pages/routes/index.tsx
import React, { useState } from 'react';
import { 
  Table, 
  TableHeader, 
  TableBody, 
  TableRow, 
  TableHead, 
  TableCell 
} from '@/components/ui/table';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Plus, Search, Edit, Trash } from 'lucide-react';

// Datos simulados
const MOCK_ROUTES = [
  {
    id: '1',
    path: '/api/users',
    targetService: 'user-service',
    method: 'GET',
    authRequired: true
  },
  {
    id: '2',
    path: '/api/products',
    targetService: 'product-service',
    method: 'POST',
    authRequired: false
  },
  {
    id: '3',
    path: '/api/orders',
    targetService: 'order-service',
    method: 'PUT',
    authRequired: true
  }
];

export function RoutesPage() {
  const [routes] = useState(MOCK_ROUTES);
  const [searchTerm, setSearchTerm] = useState('');
  const [showDialog, setShowDialog] = useState(false);
  const [selectedRoute, setSelectedRoute] = useState(null);

  const filteredRoutes = routes.filter(route =>
    route.path.toLowerCase().includes(searchTerm.toLowerCase()) ||
    route.targetService.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleDelete = (id: string) => {
    console.log('Delete route:', id);
    // Aquí iría la lógica de eliminación
  };

  const handleEdit = (route: any) => {
    setSelectedRoute(route);
    setShowDialog(true);
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Routes</h1>
        <Button onClick={() => setShowDialog(true)}>
          <Plus className="mr-2 h-4 w-4" />
          Add Route
        </Button>
      </div>

      <div className="flex items-center space-x-2">
        <Search className="h-4 w-4 text-muted-foreground" />
        <Input
          placeholder="Search routes..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="max-w-sm"
        />
      </div>

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Path</TableHead>
            <TableHead>Target Service</TableHead>
            <TableHead>Method</TableHead>
            <TableHead>Authentication</TableHead>
            <TableHead>Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {filteredRoutes.map((route) => (
            <TableRow key={route.id}>
              <TableCell>{route.path}</TableCell>
              <TableCell>{route.targetService}</TableCell>
              <TableCell>{route.method}</TableCell>
              <TableCell>
                {route.authRequired ? 'Required' : 'Not Required'}
              </TableCell>
              <TableCell>
                <div className="flex space-x-2">
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => handleEdit(route)}
                  >
                    <Edit className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => handleDelete(route.id)}
                  >
                    <Trash className="h-4 w-4" />
                  </Button>
                </div>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      {/* Aquí iría el diálogo de edición/creación */}
      {showDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
          <div className="bg-white p-6 rounded-lg">
            <h2>{selectedRoute ? 'Edit Route' : 'Add Route'}</h2>
            {/* Formulario */}
            <Button onClick={() => setShowDialog(false)}>Close</Button>
          </div>
        </div>
      )}
    </div>
  );
}

export default RoutesPage;
// src/pages/routes/index.tsx
import React, { useState, useEffect } from 'react';
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
import { routesApi } from '@/services/api/routes-api';
import { GatewayRoute } from '@/types/routes';

export function RoutesPage() {
  const [routes, setRoutes] = useState<GatewayRoute[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [showDialog, setShowDialog] = useState(false);
  const [selectedRoute, setSelectedRoute] = useState<GatewayRoute | null>(null);

  // Cargar rutas desde Kubernetes
  useEffect(() => {
    fetchRoutes();
  }, []);

  const fetchRoutes = async () => {
    try {
      setIsLoading(true);
      const response = await routesApi.getAll();
      setRoutes(response.data);
    } catch (error) {
      console.error('Error fetching routes:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const filteredRoutes = routes.filter(route =>
    route.spec.path.toLowerCase().includes(searchTerm.toLowerCase()) ||
    route.spec.targetService.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleDelete = async (name: string) => {
    if (confirm('Are you sure you want to delete this route?')) {
      try {
        await routesApi.delete(name);
        await fetchRoutes();
      } catch (error) {
        console.error('Error deleting route:', error);
      }
    }
  };

  const handleEdit = (route: GatewayRoute) => {
    setSelectedRoute(route);
    setShowDialog(true);
  };

  if (isLoading) {
    return <div>Loading routes...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Gateway Routes</h1>
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
            <TableHead>Name</TableHead>
            <TableHead>Path</TableHead>
            <TableHead>Target Service</TableHead>
            <TableHead>Method</TableHead>
            <TableHead>Authentication</TableHead>
            <TableHead>Status</TableHead>
            <TableHead>Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {filteredRoutes.map((route) => (
            <TableRow key={route.metadata.name}>
              <TableCell>{route.metadata.name}</TableCell>
              <TableCell>{route.spec.path}</TableCell>
              <TableCell>{route.spec.targetService.name}</TableCell>
              <TableCell>{route.spec.method}</TableCell>
              <TableCell>
                {route.spec.rules.auth?.required ? 'Required' : 'Not Required'}
              </TableCell>
              <TableCell>
                {route.status?.conditions?.find(c => c.type === 'Ready')?.status || 'Unknown'}
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
                    onClick={() => handleDelete(route.metadata.name)}
                  >
                    <Trash className="h-4 w-4" />
                  </Button>
                </div>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      {showDialog && (
        <RouteDialog
          route={selectedRoute}
          onClose={() => {
            setShowDialog(false);
            setSelectedRoute(null);
          }}
          onSave={async (routeData) => {
            try {
              if (selectedRoute) {
                await routesApi.update(selectedRoute.metadata.name, routeData);
              } else {
                await routesApi.create(routeData);
              }
              await fetchRoutes();
              setShowDialog(false);
              setSelectedRoute(null);
            } catch (error) {
              console.error('Error saving route:', error);
            }
          }}
        />
      )}
    </div>
  );
}

export default RoutesPage;
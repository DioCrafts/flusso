// src/components/features/routes/route-table.tsx
import React from 'react';
import { 
  Table, 
  TableHeader, 
  TableBody, 
  TableRow, 
  TableHead, 
  TableCell 
} from '../../ui/table';
import { Button } from '../../ui/button';
import { Pencil, Trash } from 'lucide-react';

interface Route {
  id: string;
  path: string;
  targetService: string;
  method: string;
  authRequired: boolean;
}

interface RouteTableProps {
  routes: Route[];
  onEdit: (route: Route) => void;
  onDelete: (id: string) => void;
}

export function RouteTable({ routes, onEdit, onDelete }: RouteTableProps) {
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Path</TableHead>
          <TableHead>Target Service</TableHead>
          <TableHead>Method</TableHead>
          <TableHead>Auth Required</TableHead>
          <TableHead>Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {routes.map(route => (
          <TableRow key={route.id}>
            <TableCell>{route.path}</TableCell>
            <TableCell>{route.targetService}</TableCell>
            <TableCell>{route.method}</TableCell>
            <TableCell>{route.authRequired ? 'Yes' : 'No'}</TableCell>
            <TableCell>
              <div className="flex space-x-2">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => onEdit(route)}
                >
                  <Pencil className="h-4 w-4" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => onDelete(route.id)}
                >
                  <Trash className="h-4 w-4" />
                </Button>
              </div>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

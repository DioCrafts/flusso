// src/components/features/routes/RouteDialog.tsx
import React, { useEffect, useState } from 'react';
import { 
  Dialog, 
  DialogContent, 
  DialogHeader, 
  DialogTitle 
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { useRoutes } from '@/hooks/useRoutes';

interface Route {
  id?: string;
  path: string;
  targetService: string;
  method: string;
  authRequired: boolean;
}

interface RouteDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  route: Route | null;
  onClose: () => void;
}

export function RouteDialog({ 
  open, 
  onOpenChange, 
  route, 
  onClose 
}: RouteDialogProps) {
  const [formData, setFormData] = useState<Route>({
    path: '',
    targetService: '',
    method: 'GET',
    authRequired: false
  });

  const { addRoute, updateRoute } = useRoutes();

  useEffect(() => {
    if (route) {
      setFormData(route);
    } else {
      setFormData({
        path: '',
        targetService: '',
        method: 'GET',
        authRequired: false
      });
    }
  }, [route]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (route?.id) {
        await updateRoute(route.id, formData);
      } else {
        await addRoute(formData);
      }
      onClose();
    } catch (error) {
      console.error('Error saving route:', error);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {route ? 'Edit Route' : 'Create Route'}
          </DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <label className="text-sm font-medium">Path</label>
            <Input
              value={formData.path}
              onChange={e => setFormData({ ...formData, path: e.target.value })}
              placeholder="/api/resource"
              required
            />
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Target Service</label>
            <Input
              value={formData.targetService}
              onChange={e => setFormData({ ...formData, targetService: e.target.value })}
              placeholder="service-name"
              required
            />
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Method</label>
            <select
              value={formData.method}
              onChange={e => setFormData({ ...formData, method: e.target.value })}
              className="w-full rounded-md border border-input bg-background px-3 py-2"
            >
              {['GET', 'POST', 'PUT', 'DELETE', 'PATCH'].map(method => (
                <option key={method} value={method}>
                  {method}
                </option>
              ))}
            </select>
          </div>

          <div className="flex items-center space-x-2">
            <input
              type="checkbox"
              id="authRequired"
              checked={formData.authRequired}
              onChange={e => setFormData({ ...formData, authRequired: e.target.checked })}
              className="rounded border-input"
            />
            <label htmlFor="authRequired" className="text-sm font-medium">
              Require Authentication
            </label>
          </div>

          <div className="flex justify-end space-x-2">
            <Button type="button" variant="outline" onClick={onClose}>
              Cancel
            </Button>
            <Button type="submit">
              {route ? 'Update' : 'Create'}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}

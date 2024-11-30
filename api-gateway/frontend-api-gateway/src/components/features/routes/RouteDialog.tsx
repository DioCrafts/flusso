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
import { GatewayRoute } from '@/types/routes';

interface RouteDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  route: GatewayRoute | null;
  onClose: () => void;
  onSave: (routeData: GatewayRoute['spec']) => Promise<void>;
}

export function RouteDialog({
  open,
  onOpenChange,
  route,
  onClose,
  onSave
}: RouteDialogProps) {
  const [formData, setFormData] = useState({
    path: '',
    targetService: {
      name: '',
      namespace: '',
      port: 80
    },
    method: 'GET' as const,
    rules: {
      auth: {
        required: false
      },
      rateLimit: {
        enabled: false,
        requestsPerSecond: 100,
        burstSize: 10
      },
      timeout: 30000,
      retry: {
        enabled: false,
        maxAttempts: 3,
        backoffMs: 1000
      }
    }
  });

  useEffect(() => {
    if (route) {
      setFormData(route.spec);
    } else {
      setFormData({
        path: '',
        targetService: {
          name: '',
          namespace: '',
          port: 80
        },
        method: 'GET',
        rules: {
          auth: {
            required: false
          },
          rateLimit: {
            enabled: false,
            requestsPerSecond: 100,
            burstSize: 10
          },
          timeout: 30000,
          retry: {
            enabled: false,
            maxAttempts: 3,
            backoffMs: 1000
          }
        }
      });
    }
  }, [route]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await onSave(formData);
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
            {route ? 'Edit Gateway Route' : 'Create Gateway Route'}
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

          <div className="space-y-4">
            <h3 className="font-medium">Target Service</h3>
            <div className="space-y-2">
              <label className="text-sm font-medium">Name</label>
              <Input
                value={formData.targetService.name}
                onChange={e => setFormData({
                  ...formData,
                  targetService: {
                    ...formData.targetService,
                    name: e.target.value
                  }
                })}
                placeholder="service-name"
                required
              />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium">Namespace</label>
              <Input
                value={formData.targetService.namespace}
                onChange={e => setFormData({
                  ...formData,
                  targetService: {
                    ...formData.targetService,
                    namespace: e.target.value
                  }
                })}
                placeholder="default"
              />
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium">Port</label>
              <Input
                type="number"
                value={formData.targetService.port}
                onChange={e => setFormData({
                  ...formData,
                  targetService: {
                    ...formData.targetService,
                    port: parseInt(e.target.value)
                  }
                })}
                placeholder="80"
              />
            </div>
          </div>

          <div className="space-y-2">
            <label className="text-sm font-medium">Method</label>
            <select
              value={formData.method}
              onChange={e => setFormData({ ...formData, method: e.target.value as any })}
              className="w-full rounded-md border border-input bg-background px-3 py-2"
            >
              {['GET', 'POST', 'PUT', 'DELETE', 'PATCH'].map(method => (
                <option key={method} value={method}>
                  {method}
                </option>
              ))}
            </select>
          </div>

          <div className="space-y-4">
            <h3 className="font-medium">Rules</h3>
            
            <div className="flex items-center space-x-2">
              <input
                type="checkbox"
                id="authRequired"
                checked={formData.rules.auth.required}
                onChange={e => setFormData({
                  ...formData,
                  rules: {
                    ...formData.rules,
                    auth: {
                      ...formData.rules.auth,
                      required: e.target.checked
                    }
                  }
                })}
                className="rounded border-input"
              />
              <label htmlFor="authRequired" className="text-sm font-medium">
                Require Authentication
              </label>
            </div>

            <div className="flex items-center space-x-2">
              <input
                type="checkbox"
                id="rateLimitEnabled"
                checked={formData.rules.rateLimit.enabled}
                onChange={e => setFormData({
                  ...formData,
                  rules: {
                    ...formData.rules,
                    rateLimit: {
                      ...formData.rules.rateLimit,
                      enabled: e.target.checked
                    }
                  }
                })}
                className="rounded border-input"
              />
              <label htmlFor="rateLimitEnabled" className="text-sm font-medium">
                Enable Rate Limiting
              </label>
            </div>

            {formData.rules.rateLimit.enabled && (
              <div className="space-y-2 pl-6">
                <div className="space-y-2">
                  <label className="text-sm font-medium">Requests per Second</label>
                  <Input
                    type="number"
                    value={formData.rules.rateLimit.requestsPerSecond}
                    onChange={e => setFormData({
                      ...formData,
                      rules: {
                        ...formData.rules,
                        rateLimit: {
                          ...formData.rules.rateLimit,
                          requestsPerSecond: parseInt(e.target.value)
                        }
                      }
                    })}
                  />
                </div>
              </div>
            )}
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
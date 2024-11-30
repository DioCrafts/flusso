// src/pages/security/index.tsx
import React, { useEffect, useState } from 'react';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle
} from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Switch } from '@/components/ui/switch';
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger
} from '@/components/ui/tabs';
import { securityApi, SecuritySettings } from '@/services/api/security-api';

export function SecurityPage() {
  const [settings, setSettings] = useState<SecuritySettings>({
    rateLimit: {
      enabled: false,
      requestsPerMinute: 0,
      burstSize: 0
    },
    authentication: {
      jwtSecret: '',
      tokenExpiration: 0,
      refreshTokenEnabled: false
    },
    cors: {
      enabled: false,
      allowedOrigins: '',
      allowedMethods: []
    }
  });
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      setIsLoading(true);
      const response = await securityApi.getSettings();
      setSettings(response.data);
      setError(null);
    } catch (err) {
      setError('Error loading settings');
      console.error('Error loading settings:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSettingChange = (section: string, key: string, value: any) => {
    setSettings(prev => ({
      ...prev,
      [section]: {
        ...prev[section as keyof typeof prev],
        [key]: value
      }
    }));
  };

  const handleSave = async () => {
    try {
      setIsLoading(true);
      await securityApi.updateSettings(settings);
      setError(null);
      await loadSettings();
    } catch (err) {
      setError('Error saving settings');
      console.error('Error saving settings:', err);
    } finally {
      setIsLoading(false);
    }
  };

  if (isLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="animate-spin h-8 w-8 border-4 border-primary border-t-transparent rounded-full" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Security Settings</h1>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative">
          {error}
        </div>
      )}

      {/* Mantén el resto de tu JSX igual */}
      <Tabs defaultValue="rate-limiting">
        {/* ... Tu código existente ... */}
      </Tabs>

      <div className="flex justify-end space-x-4">
        <Button 
          variant="outline" 
          onClick={loadSettings}
          disabled={isLoading}
        >
          Reset
        </Button>
        <Button 
          onClick={handleSave}
          disabled={isLoading}
        >
          {isLoading ? 'Saving...' : 'Save Settings'}
        </Button>
      </div>
    </div>
  );
}

export default SecurityPage;
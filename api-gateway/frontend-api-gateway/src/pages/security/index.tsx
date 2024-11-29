// src/pages/security/index.tsx
import React from 'react';
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

export function SecurityPage() {
  const [settings, setSettings] = React.useState({
    rateLimit: {
      enabled: true,
      requestsPerMinute: 100,
      burstSize: 50
    },
    authentication: {
      jwtSecret: '',
      tokenExpiration: 3600,
      refreshTokenEnabled: true
    },
    cors: {
      enabled: true,
      allowedOrigins: '*',
      allowedMethods: ['GET', 'POST', 'PUT', 'DELETE']
    }
  });

  const handleSettingChange = (section: string, key: string, value: any) => {
    setSettings(prev => ({
      ...prev,
      [section]: {
        ...prev[section as keyof typeof prev],
        [key]: value
      }
    }));
  };

  const handleSave = () => {
    // Aquí iría la lógica para guardar la configuración
    console.log('Saving settings:', settings);
  };

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Security Settings</h1>

      <Tabs defaultValue="rate-limiting">
        <TabsList>
          <TabsTrigger value="rate-limiting">Rate Limiting</TabsTrigger>
          <TabsTrigger value="authentication">Authentication</TabsTrigger>
          <TabsTrigger value="cors">CORS</TabsTrigger>
        </TabsList>

        <TabsContent value="rate-limiting">
          <Card>
            <CardHeader>
              <CardTitle>Rate Limiting Configuration</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="font-medium">Enable Rate Limiting</span>
                <Switch
                  checked={settings.rateLimit.enabled}
                  onCheckedChange={(checked) =>
                    handleSettingChange('rateLimit', 'enabled', checked)
                  }
                />
              </div>

              {settings.rateLimit.enabled && (
                <>
                  <div className="space-y-2">
                    <label className="text-sm font-medium">
                      Requests per Minute
                    </label>
                    <Input
                      type="number"
                      value={settings.rateLimit.requestsPerMinute}
                      onChange={(e) =>
                        handleSettingChange('rateLimit', 'requestsPerMinute',
                          parseInt(e.target.value)
                        )
                      }
                    />
                  </div>

                  <div className="space-y-2">
                    <label className="text-sm font-medium">
                      Burst Size
                    </label>
                    <Input
                      type="number"
                      value={settings.rateLimit.burstSize}
                      onChange={(e) =>
                        handleSettingChange('rateLimit', 'burstSize',
                          parseInt(e.target.value)
                        )
                      }
                    />
                  </div>
                </>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="authentication">
          <Card>
            <CardHeader>
              <CardTitle>Authentication Settings</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">JWT Secret</label>
                <Input
                  type="password"
                  value={settings.authentication.jwtSecret}
                  onChange={(e) =>
                    handleSettingChange('authentication', 'jwtSecret',
                      e.target.value
                    )
                  }
                />
              </div>

              <div className="space-y-2">
                <label className="text-sm font-medium">
                  Token Expiration (seconds)
                </label>
                <Input
                  type="number"
                  value={settings.authentication.tokenExpiration}
                  onChange={(e) =>
                    handleSettingChange('authentication', 'tokenExpiration',
                      parseInt(e.target.value)
                    )
                  }
                />
              </div>

              <div className="flex items-center justify-between">
                <span className="font-medium">Enable Refresh Tokens</span>
                <Switch
                  checked={settings.authentication.refreshTokenEnabled}
                  onCheckedChange={(checked) =>
                    handleSettingChange('authentication', 'refreshTokenEnabled', checked)
                  }
                />
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="cors">
          <Card>
            <CardHeader>
              <CardTitle>CORS Configuration</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="font-medium">Enable CORS</span>
                <Switch
                  checked={settings.cors.enabled}
                  onCheckedChange={(checked) =>
                    handleSettingChange('cors', 'enabled', checked)
                  }
                />
              </div>

              {settings.cors.enabled && (
                <>
                  <div className="space-y-2">
                    <label className="text-sm font-medium">
                      Allowed Origins
                    </label>
                    <Input
                      value={settings.cors.allowedOrigins}
                      onChange={(e) =>
                        handleSettingChange('cors', 'allowedOrigins',
                          e.target.value
                        )
                      }
                      placeholder="* or specific origins"
                    />
                  </div>

                  <div className="space-y-2">
                    <label className="text-sm font-medium">
                      Allowed Methods
                    </label>
                    <div className="flex flex-wrap gap-2">
                      {['GET', 'POST', 'PUT', 'DELETE'].map(method => (
                        <Button
                          key={method}
                          variant={settings.cors.allowedMethods.includes(method) ? 'default' : 'outline'}
                          onClick={() => {
                            const methods = settings.cors.allowedMethods.includes(method)
                              ? settings.cors.allowedMethods.filter(m => m !== method)
                              : [...settings.cors.allowedMethods, method];
                            handleSettingChange('cors', 'allowedMethods', methods);
                          }}
                        >
                          {method}
                        </Button>
                      ))}
                    </div>
                  </div>
                </>
              )}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      <div className="flex justify-end">
        <Button onClick={handleSave}>
          Save Settings
        </Button>
      </div>
    </div>
  );
}

export default SecurityPage;

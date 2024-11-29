// src/components/layout/navbar.tsx
import React from 'react';
import { Button } from '../ui/button';
import { Bell, Settings, User } from 'lucide-react';

export function Navbar() {
  return (
    <header className="border-b">
      <div className="flex h-16 items-center px-4">
        <div className="flex-1">
          <h1 className="text-xl font-bold">API Gateway</h1>
        </div>
        <div className="flex items-center space-x-4">
          <Button variant="ghost" size="sm">
            <Bell className="h-5 w-5" />
          </Button>
          <Button variant="ghost" size="sm">
            <Settings className="h-5 w-5" />
          </Button>
          <Button variant="ghost" size="sm">
            <User className="h-5 w-5" />
          </Button>
        </div>
      </div>
    </header>
  );
}

// src/components/layout/main-layout.tsx
import React from 'react';
import { Sidebar } from './sidebar';
import { Navbar } from './navbar';

interface MainLayoutProps {
  children: React.ReactNode;
  onPageChange: (page: string) => void;
}

export function MainLayout({ children, onPageChange }: MainLayoutProps) {
  return (
    <div className="min-h-screen bg-background">
      <Navbar />
      <div className="flex">
        <Sidebar onPageChange={onPageChange} />
        <main className="flex-1 p-8">
          {children}
        </main>
      </div>
    </div>
  );
}
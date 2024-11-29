// src/App.tsx
import React, { useEffect, useState } from 'react';
import { MainLayout } from './components/layout/main-layout';
import { LoginPage } from './pages/auth/login';
import { DashboardPage } from './pages/dashboard';
import { RoutesPage } from './pages/routes';
import { ServicesPage } from './pages/services';
import { SecurityPage } from './pages/security';
import { useAuth } from './hooks/useAuth';

export function App() {
  const [currentPage, setCurrentPage] = useState<string>('dashboard');
  const { isAuthenticated, checkAuth, isLoading } = useAuth();

  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  if (isLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="animate-spin h-8 w-8 border-4 border-primary border-t-transparent rounded-full" />
      </div>
    );
  }

  if (!isAuthenticated) {
    return <LoginPage />;
  }

  const renderContent = () => {
    switch (currentPage) {
      case 'dashboard':
        return <DashboardPage />;
      case 'routes':
        return <RoutesPage />;
      case 'services':
        return <ServicesPage />;
      case 'security':
        return <SecurityPage />;
      default:
        return <DashboardPage />;
    }
  };

  return (
    <div className="min-h-screen bg-background">
      <MainLayout onPageChange={setCurrentPage}>
        <div className="p-6">
          <div className="mx-auto max-w-7xl">
            {renderContent()}
          </div>
        </div>
      </MainLayout>
    </div>
  );
}

// Providers Wrapper
function AppWithProviders() {
  return (
    <div className="relative">
      {/* Notificaciones globales */}
      <div className="fixed top-4 right-4 z-50" id="notifications" />
      
      {/* ThemeProvider si usas temas */}
      <div className="min-h-screen">
        <App />
      </div>
    </div>
  );
}

export default AppWithProviders;

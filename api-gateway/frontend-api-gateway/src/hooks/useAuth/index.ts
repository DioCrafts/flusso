// src/hooks/useAuth/index.ts
import { useState, useCallback } from 'react';
import { UseAuthReturn, User, LoginCredentials } from './types';

const DEV_USER = {
  email: 'admin@test.com',
  password: 'admin123',
  userData: {
    id: '1',
    email: 'admin@test.com',
    role: 'admin',
    name: 'Admin User',
  },
};

export function useAuth(): UseAuthReturn {
  const [user, setUser] = useState<User | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const checkAuth = useCallback(async () => {
    const token = localStorage.getItem('token');
    if (token) {
      setUser(DEV_USER.userData); // En desarrollo, usa usuario de prueba
      setIsAuthenticated(true);
    }
  }, []);

  const login = useCallback(async (credentials: LoginCredentials) => {
    try {
      setIsLoading(true);
      setError(null);

      if (import.meta.env.DEV) {
        // Lógica para desarrollo
        if (credentials.email === DEV_USER.email && credentials.password === DEV_USER.password) {
          await new Promise((resolve) => setTimeout(resolve, 500)); // Simula un delay
          const mockToken = 'dev-token-123';
          localStorage.setItem('token', mockToken);
          setUser(DEV_USER.userData);
          setIsAuthenticated(true);
          return;
        } else {
          throw new Error('Invalid credentials');
        }
      }

      // Lógica para producción
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(credentials),
      });

      if (!response.ok) throw new Error('Login failed');

      const { user: userData, token } = await response.json();
      localStorage.setItem('token', token);
      setUser(userData);
      setIsAuthenticated(true);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Login failed'));
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  const logout = useCallback(async () => {
    try {
      setIsLoading(true);

      if (!import.meta.env.DEV) {
        const token = localStorage.getItem('token');
        if (token) {
          await fetch('/api/auth/logout', {
            method: 'POST',
            headers: {
              Authorization: `Bearer ${token}`,
            },
          });
        }
      }
    } catch (err) {
      console.error('Logout error:', err);
    } finally {
      localStorage.removeItem('token');
      setUser(null);
      setIsAuthenticated(false);
      setIsLoading(false);
    }
  }, []);

  return {
    user,
    isAuthenticated,
    isLoading,
    error,
    login,
    logout,
    checkAuth,
  };
}

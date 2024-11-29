// src/stores/auth/index.ts
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { AuthState } from './types';

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      token: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,

      login: async (email: string, password: string) => {
        set({ isLoading: true, error: null });
        try {
          const response = await fetch('/api/auth/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, password }),
          });

          if (!response.ok) throw new Error('Invalid credentials');

          const { user, token } = await response.json();
          set({ 
            user, 
            token, 
            isAuthenticated: true, 
            isLoading: false 
          });
        } catch (error) {
          set({ 
            error: error instanceof Error ? error.message : 'Login failed',
            isLoading: false,
            isAuthenticated: false,
            user: null,
            token: null
          });
        }
      },

      logout: () => {
        set({ 
          user: null, 
          token: null, 
          isAuthenticated: false,
          error: null
        });
      },

      checkAuth: async () => {
        const token = get().token;
        if (!token) {
          set({ isAuthenticated: false });
          return;
        }

        set({ isLoading: true });
        try {
          const response = await fetch('/api/auth/me', {
            headers: {
              Authorization: `Bearer ${token}`
            }
          });

          if (!response.ok) throw new Error('Session expired');

          const user = await response.json();
          set({ user, isAuthenticated: true, isLoading: false });
        } catch (error) {
          set({
            user: null,
            token: null,
            isAuthenticated: false,
            isLoading: false,
            error: error instanceof Error ? error.message : 'Authentication failed'
          });
        }
      },

      clearError: () => set({ error: null }),
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({ token: state.token }),
    }
  )
);

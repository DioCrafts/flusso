// src/hooks/useAuth/types.ts
export interface User {
  id: string;
  email: string;
  role: string;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface UseAuthReturn {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: Error | null;
  login: (credentials: LoginCredentials) => Promise<void>;
  logout: () => Promise<void>;
  checkAuth: () => Promise<void>;
}

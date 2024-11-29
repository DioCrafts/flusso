// src/types/auth.ts
export interface User {
  id: string;
  email: string;
  firstName?: string;
  lastName?: string;
  role: UserRole;
  permissions: Permission[];
  metadata: UserMetadata;
}

export type UserRole = 'admin' | 'operator' | 'viewer';

export interface Permission {
  resource: string;
  action: 'create' | 'read' | 'update' | 'delete';
  conditions?: Record<string, any>;
}

export interface UserMetadata {
  createdAt: string;
  lastLogin: string;
  lastPasswordChange: string;
  twoFactorEnabled: boolean;
}

export interface LoginRequest {
  email: string;
  password: string;
  rememberMe?: boolean;
}

export interface LoginResponse {
  user: User;
  token: string;
  refreshToken: string;
  expiresIn: number;
}

export interface TokenPayload {
  sub: string;
  email: string;
  role: UserRole;
  permissions: Permission[];
  exp: number;
  iat: number;
}

export interface AuthState {
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
  loading: boolean;
  error: string | null;
}

// Utility types for API responses
export interface ApiResponse<T> {
  success: boolean;
  data: T;
  error?: string;
  metadata?: {
    timestamp: string;
    requestId: string;
  };
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}

export interface ErrorResponse {
  code: string;
  message: string;
  details?: Record<string, any>;
  stack?: string;
}

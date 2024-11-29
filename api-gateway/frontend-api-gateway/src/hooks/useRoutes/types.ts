// src/hooks/useRoutes/types.ts
import { Route } from '@/types/routes';

export interface UseRoutesReturn {
  routes: Route[];
  isLoading: boolean;
  error: Error | null;
  fetchRoutes: () => Promise<void>;
  createRoute: (route: Omit<Route, 'id'>) => Promise<void>;
  updateRoute: (id: string, route: Partial<Route>) => Promise<void>;
  deleteRoute: (id: string) => Promise<void>;
}

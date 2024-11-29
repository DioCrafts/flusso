// src/stores/routes/types.ts
export interface Route {
  id: string;
  path: string;
  targetService: string;
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
  authRequired: boolean;
  rateLimit?: {
    requestsPerSecond: number;
    burstSize: number;
  };
}

export interface RoutesState {
  routes: Route[];
  isLoading: boolean;
  error: string | null;
  selectedRoute: Route | null;
  fetchRoutes: () => Promise<void>;
  addRoute: (route: Omit<Route, 'id'>) => Promise<void>;
  updateRoute: (id: string, route: Partial<Route>) => Promise<void>;
  deleteRoute: (id: string) => Promise<void>;
  setSelectedRoute: (route: Route | null) => void;
}

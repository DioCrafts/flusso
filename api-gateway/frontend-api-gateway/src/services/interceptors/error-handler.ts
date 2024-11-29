// src/services/interceptors/error-handler.ts
import { AxiosError } from 'axios';

export class ApiError extends Error {
  constructor(
    public status: number,
    public message: string,
    public data?: any
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

export const handleApiError = (error: AxiosError) => {
  if (error.response) {
    // Error de respuesta del servidor
    throw new ApiError(
      error.response.status,
      error.response.data?.message || 'Server error',
      error.response.data
    );
  } else if (error.request) {
    // Error de red
    throw new ApiError(
      0,
      'Network error - no response received',
      error.request
    );
  } else {
    // Error de configuración
    throw new ApiError(
      0,
      error.message,
      error.config
    );
  }
};

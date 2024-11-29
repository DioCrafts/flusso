// src/config/constants.ts
export const APP_CONFIG = {
  // Configuración general de la aplicación
  APP: {
    NAME: 'API Gateway Manager',
    DESCRIPTION: 'Management interface for API Gateway',
    SUPPORT_EMAIL: 'support@apigateway.com',
    DEFAULT_LANGUAGE: 'en',
    AVAILABLE_LANGUAGES: ['en', 'es'],
    COPYRIGHT: '© 2024 API Gateway',
  },

  // Configuración de autenticación
  AUTH: {
    TOKEN_KEY: 'auth_token',
    REFRESH_TOKEN_KEY: 'refresh_token',
    TOKEN_EXPIRY: 3600, // 1 hora en segundos
    REFRESH_TOKEN_EXPIRY: 604800, // 7 días en segundos
    LOGIN_REDIRECT: '/dashboard',
    LOGOUT_REDIRECT: '/login',
  },

  // Configuración de la API
  API: {
    VERSION: 'v1',
    TIMEOUT: 30000, // 30 segundos
    RETRY_ATTEMPTS: 3,
    RETRY_DELAY: 1000, // 1 segundo
    ENDPOINTS: {
      AUTH: '/auth',
      ROUTES: '/routes',
      SERVICES: '/services',
      METRICS: '/metrics',
      HEALTH: '/health',
    },
    HEADERS: {
      CONTENT_TYPE: 'application/json',
      ACCEPT: 'application/json',
    }
  },

  // Configuración de paginación
  PAGINATION: {
    DEFAULT_PAGE_SIZE: 10,
    PAGE_SIZES: [10, 20, 50, 100],
    MAX_PAGE_SIZE: 100,
  },

  // Configuración de la interfaz de usuario
  UI: {
    THEME: {
      DEFAULT: 'light',
      STORAGE_KEY: 'theme',
    },
    ANIMATION: {
      DURATION: 200,
      EASING: 'ease-in-out',
    },
    TOAST: {
      DURATION: 5000,
      POSITION: 'bottom-right',
    },
    MODAL: {
      TRANSITION_DURATION: 150,
    },
    DATE_FORMAT: 'YYYY-MM-DD',
    TIME_FORMAT: 'HH:mm:ss',
    DATETIME_FORMAT: 'YYYY-MM-DD HH:mm:ss',
  },

  // Configuración de monitoreo y métricas
  MONITORING: {
    REFRESH_INTERVAL: 30000, // 30 segundos
    METRICS_RETENTION: 7, // 7 días
    CHART_POINTS: 24, // Puntos en gráficas
    STATUS: {
      HEALTHY: 'healthy',
      WARNING: 'warning',
      ERROR: 'error',
    },
    THRESHOLDS: {
      CPU_WARNING: 70,
      CPU_CRITICAL: 90,
      MEMORY_WARNING: 80,
      MEMORY_CRITICAL: 95,
      LATENCY_WARNING: 1000, // 1 segundo
      LATENCY_CRITICAL: 5000, // 5 segundos
    }
  },

  // Configuración de validación
  VALIDATION: {
    PASSWORD: {
      MIN_LENGTH: 8,
      REQUIRE_UPPERCASE: true,
      REQUIRE_LOWERCASE: true,
      REQUIRE_NUMBER: true,
      REQUIRE_SPECIAL: true,
    },
    URL: {
      PROTOCOLS: ['http', 'https'],
      REQUIRE_TLD: true,
    },
  },

  // Configuración de rate limiting
  RATE_LIMIT: {
    DEFAULT_LIMIT: 100,
    WINDOW_MS: 60000, // 1 minuto
    HEADERS: true,
  },

  // Configuración de caché
  CACHE: {
    TTL: 300, // 5 minutos en segundos
    MAX_ITEMS: 100,
    STORAGE_PREFIX: 'gw_cache_',
  },

  // Configuración de logs
  LOGGING: {
    LEVEL: 'info',
    MAX_FILES: 5,
    FILE_SIZE: 5242880, // 5MB
    RETENTION: 30, // 30 días
    TYPES: {
      ERROR: 'error',
      WARN: 'warn',
      INFO: 'info',
      DEBUG: 'debug',
    }
  },

  // Constantes de errores
  ERRORS: {
    CODES: {
      UNAUTHORIZED: 401,
      FORBIDDEN: 403,
      NOT_FOUND: 404,
      VALIDATION: 422,
      SERVER_ERROR: 500,
    },
    MESSAGES: {
      DEFAULT: 'An unexpected error occurred',
      AUTH_REQUIRED: 'Authentication is required',
      INVALID_CREDENTIALS: 'Invalid credentials',
      NOT_FOUND: 'Resource not found',
      VALIDATION_FAILED: 'Validation failed',
    }
  }
} as const;

// Exportar tipos útiles basados en las constantes
export type ThemeType = keyof typeof APP_CONFIG.UI.THEME;
export type LogLevel = keyof typeof APP_CONFIG.LOGGING.TYPES;
export type ServiceStatus = keyof typeof APP_CONFIG.MONITORING.STATUS;

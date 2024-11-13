
# My Rust Gateway Project Structure

This document provides a complete overview of the project structure for the Rust-based API Gateway alternative to Kong.

## Project Structure

```plaintext
/my_rust_gateway
│
├── src/                                 # Source code of the application
│   ├── core/                            # Core application functionalities
│   │   ├── config.rs                    # Global configuration
│   │   ├── errors.rs                    # Error handling
│   │   └── types.rs                     # Shared types and structures
│   │
│   ├── api_management/                  # API management and routing
│   │   ├── routes.rs                    # Route and service definitions
│   │   ├── policies.rs                  # Access and security policies
│   │   └── service_registry.rs          # Service registry and discovery
│   │
│   ├── ingress_controller/              # Ingress controller
│   │   ├── router.rs                    # Request routing logic
│   │   ├── balancer.rs                  # Load balancing
│   │   └── parser.rs                    # YAML configuration parser
│   │
│   ├── middleware/                      # Middleware for authentication and transformations
│   │   ├── auth/                        # Authentication modules
│   │   │   ├── jwt_auth.rs              # JWT authentication
│   │   │   ├── basic_auth.rs            # Basic authentication
│   │   │   └── oauth2.rs                # OAuth2 authentication
│   │   ├── logging.rs                   # Request and traffic logging
│   │   └── rate_limiting.rs             # Rate limiting
│   │
│   ├── plugins/                         # Extensible plugins
│   │   ├── plugin_manager.rs            # Plugin manager
│   │   ├── cache.rs                     # Cache plugin
│   │   ├── monitoring.rs                # Monitoring and metrics plugin
│   │   └── custom_plugin.rs             # Example customizable plugin
│   │
│   ├── certs/                           # Certificate management (TLS/SSL)
│   │   ├── cert_manager.rs              # Certificate manager
│   │   └── cert_store.rs                # Certificate storage and renewal
│   │
│   ├── monitoring/                      # Monitoring and logging system
│   │   ├── prometheus_exporter.rs       # Prometheus exporter
│   │   └── logging.rs                   # Logging configuration
│   │
│   ├── utils/                           # Shared utilities
│   │   ├── http.rs                      # HTTP utilities
│   │   └── json.rs                      # JSON handling utilities
│   │
│   ├── dashboard/                       # Dashboard module
│   │   ├── api_gateway/                 # Dashboard for centralized API Gateway
│   │   │   ├── routes.rs                # API Gateway endpoints
│   │   │   ├── controllers.rs           # API Gateway logic
│   │   │   └── ui/                      # API Gateway frontend
│   │   │       ├── static/              # Static files (CSS, JS, images)
│   │   │       └── templates/           # HTML templates
│   │   │
│   │   ├── ingress_dashboard/           # Dashboard for Ingress Controller
│   │   │   ├── routes.rs                # Ingress Controller endpoints
│   │   │   ├── controllers.rs           # Ingress Controller logic
│   │   │   └── ui/                      # Ingress Controller frontend
│   │   │       ├── static/              # Static files
│   │   │       └── templates/           # HTML templates
│   │   │
│   │   └── shared_components/           # Shared components for dashboards
│   │       ├── auth_controls.rs         # Authentication controls
│   │       └── monitoring.rs            # Monitoring components
│   │
│   ├── main.rs                          # Main entry point of the application
│   └── lib.rs                           # Main project library
│
├── config/                              # Environment configurations
│   ├── default.yaml                     # Default configuration
│   ├── dev.yaml                         # Development configuration
│   └── prod.yaml                        # Production configuration
│
├── tests/                               # Unit and integration tests
│   ├── api_tests.rs                     # API Management module tests
│   ├── ingress_tests.rs                 # Ingress Controller tests
│   ├── middleware_tests.rs              # Middleware tests
│   ├── plugins_tests.rs                 # Plugin tests
│   └── certs_tests.rs                   # Certificate management tests
│
├── public/                              # Public frontend files
│   ├── css/                             # CSS files
│   ├── js/                              # JavaScript files
│   └── index.html                       # Main HTML file
│
├── scripts/                             # CI/CD and automation scripts
│   ├── deploy.sh                        # Deployment script
│   ├── setup_env.sh                     # Environment setup script
│   └── test.sh                          # Test execution script
│
├── docs/                                # Project documentation
│   ├── architecture.md                  # Architecture documentation
│   ├── api_docs.md                      # API documentation
│   └── user_guide.md                    # User guide
│
├── .gitignore                           # Gitignore file for Rust and temp files
├── Cargo.toml                           # Rust dependency configuration file
└── README.md                            # Main project README
```

## Project Structure Overview

- **src/**: Main source code organized by functionality.
  - **core/**: Defines core configurations, types, and error handling.
  - **api_management/**: Manages routes and security policies at the API Gateway level.
  - **ingress_controller/**: Handles routing and load balancing for the Ingress Controller.
  - **middleware/**: Provides authentication, authorization, logging, and rate limiting features.
  - **plugins/**: Extensible plugins system, with manager and example plugins (cache, monitoring).
  - **certs/**: Manages TLS/SSL certificates and their storage.
  - **monitoring/**: Handles metrics and logging.
  - **utils/**: Shared utilities like JSON handling and HTTP processing.
  - **dashboard/**: Frontend and backend for API Gateway and Ingress Controller dashboards.

- **config/**: Environment-specific configurations (development, production).

- **tests/**: Unit and integration tests, separated by functionality.

- **public/**: Static files and frontend for dashboards.

- **scripts/**: Automation scripts for CI/CD.

- **docs/**: Documentation files, including architecture, API docs, and user guide.

This modular structure is designed to improve maintainability, scalability, and separation of concerns within the project, with clearly defined responsibilities across different modules.

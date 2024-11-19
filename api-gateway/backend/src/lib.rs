pub mod gateway; // Módulo para Gateway
pub mod backends; // Módulo para manejar Backends
pub mod observability; // Módulo para Observability
pub mod security; // Módulo para Seguridad
pub mod plugins; // Módulo para Plugins
pub mod rest; // REST server

// Reexportar configuraciones de rutas para `main.rs`
// pub use backends::configure_routes as configure_backends_routes;
// pub use observability::configure_routes as configure_observability_routes;
// pub use security::configure_routes as configure_security_routes;
// pub use plugins::configure_routes as configure_plugins_routes;

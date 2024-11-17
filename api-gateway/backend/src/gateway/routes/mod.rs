//! src/gateway/routes/mod.rs
//! Gateway Routes Module
//!
//! This module provides different types of route configurations for the Gateway API in Flusso.
//! Each route type (e.g., HTTPRoute, GRPCRoute, TLSRoute) enables flexible routing capabilities
//! to match and forward traffic based on protocol-specific criteria.

//! # Ventajas de la Organización de Módulos en `routes`
//!
//! Este módulo `routes` unifica todos los tipos de rutas disponibles en Flusso, facilitando la gestión y
//! organización de los diferentes tipos de rutas compatibles con el Gateway API.
//!
//! ## Ventajas de Esta Organización
//!
//! - **Consistencia**: Al unificar todos los tipos de rutas en `routes`, simplificamos la estructura del proyecto y evitamos duplicidad de responsabilidades.
//! - **Modularidad**: Cada tipo de ruta (HTTP, GRPC, TLS) tiene su propio archivo, lo cual facilita el mantenimiento y futuras expansiones del proyecto.
//! - **Escalabilidad**: Si en el futuro se desean añadir más tipos de rutas, como `WebSocketRoute` o `TCPRoute`, simplemente se pueden crear nuevos archivos dentro de `routes`, siguiendo el mismo patrón.
//!
//! Esta organización modular ayuda a mantener el proyecto limpio, organizado y fácilmente ampliable.


pub mod grpc_route;
pub mod http_route;
pub mod tls_route;

use grpc_route::GRPCRouteManager;
use http_route::HTTPRouteManager;
use tls_route::TLSRouteManager;

/// Enum to represent the different types of routes supported by Flusso's Gateway API integration.
pub enum RouteType {
    HTTP,
    GRPC,
    TLS,
}

/// A unified trait to define shared behaviors for different route managers.
///
/// This trait can be implemented by each route manager to standardize functionality,
/// such as listing, creating, and deleting routes.
pub trait RouteManager {
    type Route;

    /// Lists all routes of this type in the Kubernetes cluster.
    fn list_routes(&self) -> Vec<Self::Route>;

    /// Creates a new route of this type in the Kubernetes cluster.
    fn create_route(&self, route: &Self::Route) -> Result<Self::Route, String>;

    /// Deletes a route by name in the Kubernetes cluster.
    fn delete_route(&self, name: &str) -> Result<(), String>;
}

//! src/gateway/mod.rs
//!
//! Gateway Module for Flusso
//!
//! This module provides an interface for managing Gateway resources in Kubernetes,
//! including CRUD operations, TLS configuration, and route management.

pub mod handlers; // Define los controladores de endpoints
pub mod models; // Define las estructuras de datos compartidas
pub mod crd; // Interacción con los CRDs en Kubernetes

pub use handlers::*;
pub use handlers::{start_gateway_api, list_gateways, add_gateway, delete_gateway, configure_tls, get_gateway_metrics};
pub use models::*;
pub use crd::*;

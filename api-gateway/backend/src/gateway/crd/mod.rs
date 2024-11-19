//! Gateway CRD Module
//!
//! Este módulo proporciona todas las funcionalidades para gestionar los recursos personalizados
//! (Custom Resource Definitions - CRDs) relacionados con el Gateway API, como Gateways,
//! GatewayClass, HTTPRoute, TLSRoute y GRPCRoute.

pub mod gateway;       // Módulo para gestionar Gateway CRD
pub mod gateway_class; // Módulo para gestionar GatewayClass CRD
pub mod http_route;    // Módulo para gestionar HTTPRoute CRD
pub mod grpc_route;    // Módulo para gestionar GRPCRoute CRD
pub mod tls_route;     // Módulo para gestionar TLSRoute CRD

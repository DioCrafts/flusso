//! src/gateway/models.rs
//! 
//! Modelos compartidos para representar Gateways y rutas.

use serde::{Deserialize, Serialize};

/// Representa un Gateway en Kubernetes.
/// Este modelo mapea directamente los datos necesarios para un Gateway CRD.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gateway {
    pub id: String,                  // Identificador único del Gateway
    pub hostname: String,            // Hostname del Gateway
    pub tls_enabled: bool,           // Indica si TLS está habilitado
    pub certificate: Option<String>, // Certificado TLS, si aplica
    pub routes: Vec<Route>,          // Rutas asociadas a este Gateway
    pub gateway_class_name: String,  // Nombre de la clase del Gateway
    pub listeners: Vec<Listener>,    // Listeners del Gateway
}

impl Gateway {
    /// Convierte un `Gateway` en un `GatewaySpec` para su uso con los CRDs.
    pub fn to_spec(&self) -> super::crd::gateway::GatewaySpec {
        super::crd::gateway::GatewaySpec {
            spec: super::crd::gateway::GatewayInnerSpec {
                hostname: self.hostname.clone(),
                tls_enabled: self.tls_enabled,
                certificate: self.certificate.clone(),
                routes: self.routes.iter().map(|r| r.clone().into()).collect(),
            },
        }
    }
}


/// Representa un listener dentro de un Gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listener {
    pub name: String,     // Nombre del listener
    pub protocol: String, // Protocolo del listener (e.g., HTTP, HTTPS)
    pub port: u16,        // Puerto del listener
}

/// Representa una ruta dentro de un Gateway.
/// Este modelo mapea las configuraciones necesarias para HTTPRoute, GRPCRoute, o TLSRoute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,          // Identificador único de la ruta
    pub path: Option<String>, // Ruta específica (e.g., "/api/*")
    pub backend: String,     // Nombre del backend asociado
    pub methods: Vec<String>, // Métodos permitidos (e.g., ["GET", "POST"])
    pub protocols: Vec<String>, // Protocolos soportados (e.g., ["HTTP", "HTTPS"])
}

/// Representa métricas relacionadas con los Gateways.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayMetric {
    pub gateway_id: String,    // ID del Gateway asociado
    pub metric_name: String,   // Nombre de la métrica (e.g., "latency", "requests_per_second")
    pub value: f64,            // Valor de la métrica
    pub timestamp: String,     // Marca de tiempo de la métrica
}

/// Representa la configuración de TLS para un Gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSConfig {
    pub certificate: String,     // Contenido del certificado TLS
    pub private_key: Option<String>, // Clave privada asociada, si es necesaria
    pub ca_bundle: Option<String>,   // Certificados de autoridad, si aplica
}

/// Representa errores genéricos relacionados con los Gateways o rutas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayError {
    pub message: String,        // Descripción del error
    pub details: Option<String>, // Detalles adicionales sobre el error
}

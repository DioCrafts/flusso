//! Handlers para la gestión de seguridad en el API Gateway.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Modelo de una política de seguridad
#[derive(Serialize, Deserialize, Clone)]
pub struct SecurityPolicy {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub rules: Vec<String>, // Ejemplo: ["DDoS Protection", "SQL Injection"]
}

// Modelo para autenticación JWT
#[derive(Serialize, Deserialize, Clone)]
pub struct JwtValidationResponse {
    pub valid: bool,
    pub message: String,
}

// Estado compartido
pub struct AppState {
    pub policies: Mutex<Vec<SecurityPolicy>>, // Políticas de seguridad simuladas
}

impl AppState {
    pub fn new() -> Self {
        Self {
            policies: Mutex::new(vec![
                SecurityPolicy {
                    id: 1,
                    name: "CORS Policy".to_string(),
                    description: "Restricts access to specific origins.".to_string(),
                    active: true,
                    rules: vec!["Allow-Origin: https://example.com".to_string()],
                },
                SecurityPolicy {
                    id: 2,
                    name: "Rate Limiting".to_string(),
                    description: "Limits requests to 100 per minute.".to_string(),
                    active: true,
                    rules: vec!["Limit: 100 requests/minute".to_string()],
                },
            ]),
        }
    }
}

// Endpoints

/// Listar todas las políticas de seguridad
pub async fn list_policies(data: web::Data<AppState>) -> impl Responder {
    let policies = data.policies.lock().unwrap();
    HttpResponse::Ok().json(&*policies)
}

/// Agregar una nueva política
pub async fn add_policy(data: web::Data<AppState>, policy: web::Json<SecurityPolicy>) -> impl Responder {
    let mut policies = data.policies.lock().unwrap();
    let mut new_policy = policy.into_inner();
    new_policy.id = policies.last().map_or(1, |p| p.id + 1); // Generar un nuevo ID
    policies.push(new_policy);
    HttpResponse::Created().finish()
}

/// Eliminar una política de seguridad
pub async fn delete_policy(data: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
    let mut policies = data.policies.lock().unwrap();
    let initial_len = policies.len();
    policies.retain(|policy| policy.id != id.into_inner());
    if policies.len() < initial_len {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

/// Validar un token JWT
pub async fn validate_jwt(token: web::Path<String>) -> impl Responder {
    // Simulación de validación de JWT
    let valid = token == "valid_token";
    if valid {
        HttpResponse::Ok().json(JwtValidationResponse {
            valid: true,
            message: "Token is valid.".to_string(),
        })
    } else {
        HttpResponse::Unauthorized().json(JwtValidationResponse {
            valid: false,
            message: "Invalid token.".to_string(),
        })
    }
}

/// Obtener configuración TLS
pub async fn get_tls_config() -> impl Responder {
    // Simulación de configuración TLS
    let tls_config = serde_json::json!({
        "enabled": true,
        "certificate": "example-cert.pem",
        "key": "example-key.pem",
    });
    HttpResponse::Ok().json(tls_config)
}

/// Actualizar configuración TLS
pub async fn update_tls_config(tls_data: web::Json<(String, String)>) -> impl Responder {
    let (cert, key) = tls_data.into_inner();
    // Aquí se procesaría la actualización real del certificado
    HttpResponse::Ok().json(serde_json::json!({
        "message": "TLS configuration updated successfully",
        "certificate": cert,
        "key": key,
    }))
}

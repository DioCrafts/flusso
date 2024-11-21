//src/backends/handlers.rs
//! Handlers para la gestión de backends en el API Gateway.

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Estructura del backend
#[derive(Serialize, Deserialize, Clone)]
pub struct Backend {
    pub id: u32,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub status: String, // Healthy, Unhealthy, Degraded
    pub weight: Option<u32>, // Para balanceo de carga ponderado
}

// Estado compartido del servidor
pub struct AppState {
    pub backends: Mutex<Vec<Backend>>,
}

// Listar todos los backends
pub async fn list_backends(data: web::Data<AppState>) -> impl Responder {
    let backends = data.backends.lock().unwrap();
    HttpResponse::Ok().json(&*backends)
}

// Agregar un nuevo backend
pub async fn add_backend(data: web::Data<AppState>, backend: web::Json<Backend>) -> impl Responder {
    let mut backends = data.backends.lock().unwrap();
    let mut new_backend = backend.into_inner();
    // Generar ID único para el nuevo backend
    new_backend.id = backends.last().map_or(1, |b| b.id + 1);
    new_backend.status = "Unknown".to_string(); // Inicialmente desconocido
    backends.push(new_backend);
    HttpResponse::Created().finish()
}

// Actualizar un backend existente
pub async fn update_backend(
    data: web::Data<AppState>,
    id: web::Path<u32>,
    updated_backend: web::Json<Backend>,
) -> impl Responder {
    let mut backends = data.backends.lock().unwrap();
    let id = id.into_inner();
    if let Some(backend) = backends.iter_mut().find(|b| b.id == id) {
        backend.name = updated_backend.name.clone();
        backend.address = updated_backend.address.clone();
        backend.port = updated_backend.port;
        backend.weight = updated_backend.weight;
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Eliminar un backend por ID
pub async fn delete_backend(data: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
    let mut backends = data.backends.lock().unwrap();
    let id = id.into_inner();
    let initial_len = backends.len();
    backends.retain(|backend| backend.id != id);
    if backends.len() < initial_len {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Health Check de los backends
pub async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let mut backends = data.backends.lock().unwrap();
    for backend in backends.iter_mut() {
        let url = format!("http://{}:{}", backend.address, backend.port);
        if let Ok(response) = reqwest::get(&url).await {
            backend.status = if response.status().is_success() {
                "Healthy".to_string()
            } else {
                "Unhealthy".to_string()
            };
        } else {
            backend.status = "Unhealthy".to_string();
        }
    }
    HttpResponse::Ok().json(&*backends)
}

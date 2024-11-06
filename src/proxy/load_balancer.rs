use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)] // Añade Clone y Debug aquí
pub struct LoadBalancer {
    backends: Arc<Mutex<Vec<SocketAddr>>>, // Compartido de manera segura y accesible para modificaciones concurrentes
    current_index: Arc<Mutex<usize>>,
}

impl LoadBalancer {
    pub fn new(backends: Vec<SocketAddr>) -> Self {
        Self {
            backends: Arc::new(Mutex::new(backends)),
            current_index: Arc::new(Mutex::new(0)),
        }
    }

    /// Selecciona el siguiente backend de acuerdo al algoritmo Round Robin.
    pub fn select_backend(&self) -> Option<SocketAddr> {
        let backends = self.backends.lock().unwrap();
        println!("Backends disponibles en el balanceador: {:?}", *backends); // Log para ver backends actuales
        if backends.is_empty() {
            println!("No hay backends disponibles");
            return None;
        }
        let mut index = self.current_index.lock().unwrap();
        let backend = backends[*index];
        *index = (*index + 1) % backends.len();
        println!("Backend seleccionado: {}", backend); // Log detallado al seleccionar un backend
        Some(backend)
    }

    /// Permite añadir un backend en caliente.
    pub fn add_backend(&self, backend: SocketAddr) {
        let mut backends = self.backends.lock().unwrap();
        if !backends.contains(&backend) {
            println!("Agregando backend: {}", backend); // Log detallado al añadir
            backends.push(backend);
        } else {
            println!("Backend ya existe: {}", backend);
        }
    }

    /// Permite remover un backend.
    pub fn remove_backend(&self, backend: &SocketAddr) {
        let mut backends = self.backends.lock().unwrap();
        backends.retain(|&b| b != *backend);
    }

    /// Devuelve una lista de los backends actuales.
    pub fn get_backends(&self) -> Vec<SocketAddr> {
        let backends = self.backends.lock().unwrap();
        backends.clone() // Retorna una copia de la lista de backends
    }
}


// src/services/discovery.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, Error};
use k8s_openapi::api::core::v1::Service;
use std::time::Duration;
use reqwest;
use futures::{StreamExt, TryStreamExt, TryFutureExt};
use kube::{
    api::{Api, ListParams, WatchParams, WatchEvent},
    runtime::watcher::{watcher, Config},
    Client, ResourceExt,
};
use super::{ServiceInstance, ServiceStatus};
use crate::models::kubernetes::GatewayRoute;

pub struct ServiceDiscovery {
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
    routes: Arc<RwLock<HashMap<String, GatewayRoute>>>,
    client: Client,
}

impl ServiceDiscovery {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::try_default().await.map_err(Error::from)?;
        Ok(Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            routes: Arc::new(RwLock::new(HashMap::new())),
            client,
        })
    }

    // Método existente para servicios
    pub async fn start_discovery(&self) -> Result<()> {
        let services: Api<Service> = Api::all(self.client.clone());
        let wp = WatchParams::default();

        let mut stream = services.watch(&wp, "0").await?.boxed();
        while let Some(status) = stream.try_next().await? {
            match status {
                WatchEvent::Added(svc) | WatchEvent::Modified(svc) => {
                    self.register_service(&svc).await?;
                }
                WatchEvent::Deleted(svc) => {
                    self.deregister_service(&svc).await?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    // Nuevo método para GatewayRoutes
    pub async fn watch_gateway_routes(&self) -> Result<()> {
        let routes: Api<GatewayRoute> = Api::all(self.client.clone());
        let wp = WatchParams::default();

        let mut stream = routes.watch(&wp, "0").await?.boxed();
        while let Some(event) = stream.try_next().await? {
            match event {
                WatchEvent::Added(route) => self.handle_route_added(route).await?,
                WatchEvent::Modified(route) => self.handle_route_modified(route).await?,
                WatchEvent::Deleted(route) => self.handle_route_deleted(route).await?,
                _ => {}
            }
        }
        Ok(())
    }

    async fn handle_route_added(&self, route: GatewayRoute) -> Result<()> {
        let mut routes = self.routes.write().await;
        let route_name = route.metadata.name.clone().unwrap_or_default();
        routes.insert(route_name, route);
        // Aquí podrías añadir lógica adicional como actualizar el proxy, etc.
        Ok(())
    }

    async fn handle_route_modified(&self, route: GatewayRoute) -> Result<()> {
        let mut routes = self.routes.write().await;
        let route_name = route.metadata.name.clone().unwrap_or_default();
        routes.insert(route_name, route);
        // Aquí podrías añadir lógica para actualizar configuraciones existentes
        Ok(())
    }

    async fn handle_route_deleted(&self, route: GatewayRoute) -> Result<()> {
        let mut routes = self.routes.write().await;
        let route_name = route.metadata.name.clone().unwrap_or_default();
        routes.remove(&route_name);
        // Aquí podrías añadir lógica para limpiar recursos
        Ok(())
    }

    // Método para obtener todas las rutas actuales
    pub async fn get_gateway_routes(&self) -> Vec<GatewayRoute> {
        let routes = self.routes.read().await;
        routes.values().cloned().collect()
    }

    pub async fn register_service(&self, k8s_service: &Service) -> Result<()> {
        let mut services = self.services.write().await;
        let service_name = k8s_service.metadata.name.clone().unwrap_or_default();
        let service_namespace = k8s_service.metadata.namespace.clone().unwrap_or_default();

        let labels = k8s_service.metadata.labels.clone().unwrap_or_default();

        let instance = ServiceInstance {
            id: format!("{}-{}", service_namespace, service_name),
            name: service_name.clone(),
            url: self.get_service_url(k8s_service),
            health_check_url: format!("{}/health", self.get_service_url(k8s_service)),
            status: ServiceStatus::Unknown,
            metadata: super::ServiceMetadata {
                version: labels.get("version")
                    .cloned()
                    .unwrap_or_default(),
                environment: labels.get("environment")
                    .cloned()
                    .unwrap_or_default(),
                tags: vec![],
            },
            last_check: chrono::Utc::now(),
        };

        services
            .entry(service_name)
            .or_insert_with(Vec::new)
            .push(instance);

        Ok(())
    }

    pub async fn deregister_service(&self, k8s_service: &Service) -> Result<()> {
        let mut services = self.services.write().await;
        let service_name = k8s_service.metadata.name.clone().unwrap_or_default();
        services.remove(&service_name);
        Ok(())
    }

    pub async fn get_service(&self, name: &str) -> Option<Vec<ServiceInstance>> {
        self.services.read().await.get(name).cloned()
    }

    pub async fn health_check(&self) -> Result<()> {
        let client = reqwest::Client::new();
        let mut services = self.services.write().await;

        for instances in services.values_mut() {
            for instance in instances.iter_mut() {
                let status = match client
                    .get(&instance.health_check_url)
                    .timeout(Duration::from_secs(5))
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => ServiceStatus::Healthy,
                    _ => ServiceStatus::Unhealthy,
                };

                instance.status = status;
                instance.last_check = chrono::Utc::now();
            }
        }

        Ok(())
    }

    fn get_service_url(&self, service: &Service) -> String {
        format!(
            "http://{}:{}",
            service.metadata.name.clone().unwrap_or_default(),
            service
                .spec
                .as_ref()
                .and_then(|spec| spec.ports.as_ref())
                .and_then(|ports| ports.first())
                .map(|port| port.port)
                .unwrap_or(80)
        )
    }

}

// En otro archivo, implementar el inicio de ambos watchers
pub async fn start_watchers(discovery: Arc<ServiceDiscovery>) -> Result<()> {
    // Usar tokio::spawn para ejecutar ambos watchers concurrentemente
    let service_watcher = tokio::spawn({
        let discovery = discovery.clone();
        async move { discovery.start_discovery().await }
    });

    let route_watcher = tokio::spawn({
        let discovery = discovery.clone();
        async move { discovery.watch_gateway_routes().await }
    });

    // Esperar a que ambos watchers completen o manejar errores
    let (service_result, route_result) = tokio::try_join!(
        service_watcher.map_err(|e| anyhow::anyhow!("Service watcher error: {}", e)),
        route_watcher.map_err(|e| anyhow::anyhow!("Route watcher error: {}", e))
    )?;
    
    if let Err(e) = service_result {
        return Err(anyhow::anyhow!("Service watcher failed: {}", e));
    }
    
    if let Err(e) = route_result {
        return Err(anyhow::anyhow!("Route watcher failed: {}", e));
    }
    
    Ok(())
    
}
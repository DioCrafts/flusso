// src/services/discovery.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, Error};
use k8s_openapi::api::core::v1::Service;
use std::time::Duration;
use reqwest;
use futures::{StreamExt, TryStreamExt};  // Añadido TryStreamExt
use kube::{
    api::{Api, ListParams, WatchParams, WatchEvent},
    Client, ResourceExt,
};
use super::{ServiceInstance, ServiceStatus};

pub struct ServiceDiscovery {
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
    client: Client,
}

impl ServiceDiscovery {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::try_default().await.map_err(Error::from)?;
        Ok(Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            client,
        })
    }

    pub async fn start_discovery(&self) -> Result<()> {
        let services: Api<Service> = Api::all(self.client.clone());
        let wp = WatchParams::default();  // Cambiado de ListParams a WatchParams

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

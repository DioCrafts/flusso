// src/services/load_balancer.rs
use std::sync::Arc;
use std::collections::HashMap;  // Añadido
use rand::seq::SliceRandom;
use tokio::sync::RwLock;
use super::{ServiceInstance, ServiceStatus};
use anyhow::{Result, anyhow};
use serde::Serialize;  // Añadido

pub enum LoadBalancingStrategy {
    RoundRobin,
    Random,
    LeastConnections,
}

pub struct LoadBalancer {
    discovery: Arc<super::discovery::ServiceDiscovery>,
    strategy: LoadBalancingStrategy,
    connections: Arc<RwLock<HashMap<String, usize>>>,
}

impl LoadBalancer {
    pub fn new(
        discovery: Arc<super::discovery::ServiceDiscovery>,
        strategy: LoadBalancingStrategy,
    ) -> Self {
        Self {
            discovery,
            strategy,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_service(&self, service_name: &str) -> Result<ServiceInstance> {
        let instances = self.discovery
            .get_service(service_name)
            .await
            .ok_or_else(|| anyhow!("Service not found"))?;

        let healthy_instances: Vec<&ServiceInstance> = instances
            .iter()
            .filter(|instance| matches!(instance.status, ServiceStatus::Healthy))
            .collect();

        if healthy_instances.is_empty() {
            return Err(anyhow!("No healthy instances available"));
        }

        match self.strategy {
            LoadBalancingStrategy::Random => {
                let mut rng = rand::thread_rng();
                healthy_instances
                    .choose(&mut rng)
                    .map(|&instance| instance.clone())
                    .ok_or_else(|| anyhow!("Failed to select random instance"))
            }
            LoadBalancingStrategy::RoundRobin => {
                let mut connections = self.connections.write().await;
                let current = connections
                    .entry(service_name.to_string())
                    .or_insert(0);
                *current = (*current + 1) % healthy_instances.len();
                Ok(healthy_instances[*current].clone())
            }
            LoadBalancingStrategy::LeastConnections => {
                let connections = self.connections.read().await;
                healthy_instances
                    .iter()
                    .min_by_key(|instance| {
                        connections
                            .get(&instance.id)
                            .copied()
                            .unwrap_or(0)
                    })
                    .map(|&instance| instance.clone())
                    .ok_or_else(|| anyhow!("Failed to find instance with least connections"))
            }
        }
    }

    pub async fn increment_connections(&self, service_id: &str) {
        let mut connections = self.connections.write().await;
        *connections.entry(service_id.to_string()).or_insert(0) += 1;
    }

    pub async fn decrement_connections(&self, service_id: &str) {
        let mut connections = self.connections.write().await;
        if let Some(count) = connections.get_mut(service_id) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    pub async fn get_service_metrics(&self, service_name: &str) -> Result<ServiceMetrics> {
        let instances = self.discovery
            .get_service(service_name)
            .await
            .ok_or_else(|| anyhow!("Service not found"))?;

        let total = instances.len();
        let healthy = instances
            .iter()
            .filter(|i| matches!(i.status, ServiceStatus::Healthy))
            .count();

        Ok(ServiceMetrics {
            total_instances: total,
            healthy_instances: healthy,
            active_connections: self.connections
                .read()
                .await
                .get(service_name)
                .copied()
                .unwrap_or(0),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ServiceMetrics {
    pub total_instances: usize,
    pub healthy_instances: usize,
    pub active_connections: usize,
}
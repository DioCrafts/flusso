//! src/gateway/mod.rs

//! Gateway Module for Flusso
//!
//! This module serves as the main interface for managing Gateway API resources within Flusso.
//! It provides functionality for managing Gateway, GatewayClass, and various route resources
//! (e.g., HTTPRoute, TLSRoute, GRPCRoute), allowing advanced network configuration in Kubernetes.

use kube::Client;
use kube::api::ListParams;
use std::sync::Arc;
use tokio::sync::RwLock;

mod gateway;
mod gateway_class;
pub mod routes;

use gateway::{GatewayManager, Gateway, GatewayError};
use gateway_class::GatewayClassManager;
use routes::{http_route::HTTPRouteManager, tls_route::TLSRouteManager, grpc_route::GRPCRouteManager};

// Import the trait RouteManager to ensure methods are in scope.
use crate::gateway::routes::tls_route::RouteManager;
use crate::gateway::routes::grpc_route::GRPCRouteError;

/// Manages all resources related to the Gateway API, providing an interface to create, list,
/// and delete Gateways, GatewayClasses, and Routes.
pub struct GatewayApiController {
    gateway_manager: GatewayManager,
    gateway_class_manager: GatewayClassManager,
    http_route_manager: HTTPRouteManager,
    tls_route_manager: TLSRouteManager,
    grpc_route_manager: GRPCRouteManager,
}

impl GatewayApiController {
    /// Creates a new instance of `GatewayApiController`, initializing each manager.
    pub fn new(client: Client) -> Self {
        GatewayApiController {
            gateway_manager: GatewayManager::new(client.clone()),
            gateway_class_manager: GatewayClassManager::new(client.clone()),
            http_route_manager: HTTPRouteManager::new(client.clone()),
            tls_route_manager: TLSRouteManager::new(client.clone()),
            grpc_route_manager: GRPCRouteManager::new(client),
        }
    }

    /// Lists all Gateways in the Kubernetes cluster.
    pub async fn list_gateways(&self, namespace: &str) {
        let gateways = self.gateway_manager.list_gateways(namespace).await;
        match gateways {
            Ok(gateways) => println!("Gateways in {}: {:?}", namespace, gateways),
            Err(e) => eprintln!("Failed to list Gateways in {}: {:?}", namespace, e),
        }
    }

    /// Lists all GatewayClasses in the Kubernetes cluster.
    pub async fn list_gateway_classes(&self) {
        let gateway_classes = self.gateway_class_manager.list_gateway_classes().await;
        println!("GatewayClasses: {:?}", gateway_classes);
    }

    /// Lists all HTTPRoute resources.
    pub async fn list_http_routes(&self) {
        let http_routes = self.http_route_manager.list_http_routes("default").await;
        println!("HTTPRoutes: {:?}", http_routes);
    }

    /// Lists all TLSRoute resources.
    pub async fn list_tls_routes(&self) {
        let tls_routes = self.tls_route_manager.list_routes().await;
        println!("TLSRoutes: {:?}", tls_routes);
    }

    /// Lists all GRPCRoute resources.
    pub async fn list_grpc_routes(&self, namespace: &str) -> Result<(), GRPCRouteError> {
        let grpc_routes = self.grpc_route_manager.list_grpc_routes(namespace).await?;
        println!("GRPCRoutes: {:?}", grpc_routes);
        Ok(())
    }    

    /// Creates a new Gateway based on the provided configuration.
    pub async fn create_gateway(&self, namespace: &str, gateway: &Gateway) {
        match self.gateway_manager.create_gateway(namespace, &gateway.spec).await {
            Ok(_) => println!("Gateway created successfully in {}.", namespace),
            Err(e) => eprintln!("Failed to create Gateway in {}: {}", namespace, e),
        }
    }    

    /// Deletes a Gateway by name.
    pub async fn delete_gateway(&self, namespace: &str, name: &str) {
        match self.gateway_manager.delete_gateway(namespace, name).await {
            Ok(_) => println!("Gateway deleted successfully in {}.", namespace),
            Err(e) => eprintln!("Failed to delete Gateway in {}: {}", namespace, e),
        }
    } 
  
}

/// Starts the Gateway API Controller asynchronously, providing an interface to manage Gateway resources.
/// This function sets up the controller and logs any errors encountered during runtime.
pub async fn start_gateway_api(client: Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let controller = Arc::new(RwLock::new(GatewayApiController::new(client)));

    // Example usage: List Gateways in a specific namespace
    controller.read().await.list_gateways("default").await;
    // controller.read().await.list_gateways(namespace).await;
    
    // Additional tasks or event loops related to Gateway API management can be added here.
    println!("Gateway API started successfully.");

    Ok(())
}
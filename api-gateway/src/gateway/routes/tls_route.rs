// src/gateway/routes/tls_route.rs

//! Module for managing TLSRoute resources in Kubernetes using the Gateway API.

use kube::{Api, Client, CustomResourceExt};
use kube::api::{PostParams, DeleteParams, ListParams, ApiResource, DynamicObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kube_derive::CustomResource; // Import the CustomResource derive macro
use async_trait::async_trait;
use std::error::Error;
use tokio::time::{Duration, sleep};
use kube::core::GroupVersionKind;


/// Define `TLSRouteInnerSpec` for TLS route parameters.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TLSRouteInnerSpec {
    pub sni_hosts: Vec<String>,
    pub backend_refs: Vec<BackendReference>,
}

/// BackendReference for destination backends.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BackendReference {
    pub name: String,
    pub port: u16,
}

/// Define the `TLSRoute` CRD for TLS routing.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "networking.flusso.io",
    version = "v1alpha1",
    kind = "TLSRoute",
    namespaced
)]
pub struct TLSRouteSpec {
    pub spec: TLSRouteInnerSpec,
}

/// TLSRouteManager manages the lifecycle of TLSRoute.
pub struct TLSRouteManager {
    client: Client,
    ar: ApiResource, // Define ApiResource for TLSRoute
}

impl TLSRouteManager {
    pub fn new(client: Client) -> Self {
        // Crea un GroupVersionKind con el grupo, versión y tipo del recurso
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "Gateway");

        // Define el ApiResource para el CRD personalizado Gateway
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
    }    

}

/// Define the RouteManager trait that TLSRouteManager should implement.
#[async_trait]
pub trait RouteManager {
    type Route;

    async fn list_routes(&self) -> Vec<Self::Route>;
    async fn create_route(&self, route: &Self::Route) -> Result<Self::Route, String>;
    async fn delete_route(&self, name: &str) -> Result<(), String>;
}

#[async_trait]
impl RouteManager for TLSRouteManager {
    type Route = DynamicObject;

    async fn list_routes(&self) -> Vec<Self::Route> {
        let api: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        match api.list(&ListParams::default()).await {
            Ok(routes) => routes.items,
            Err(e) => {
                eprintln!("Error listing TLSRoutes: {:?}", e);
                vec![]
            }
        }
    }

    async fn create_route(&self, route: &Self::Route) -> Result<Self::Route, String> {
        let api: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        match api.create(&PostParams::default(), route).await {
            Ok(created_route) => Ok(created_route),
            Err(e) => Err(format!("Failed to create TLSRoute: {:?}", e)),
        }
    }

    async fn delete_route(&self, name: &str) -> Result<(), String> {
        let api: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        match api.delete(name, &DeleteParams::default()).await {
            Ok(_) => {
                // Delay to ensure deletion completes
                sleep(Duration::from_secs(2)).await;
                Ok(())
            }
            Err(e) => Err(format!("Failed to delete TLSRoute: {:?}", e)),
        }
    }
}

//! src/gateway/crd/tls_route.rs
//!
//! Module for managing TLSRoute resources in Kubernetes using the Gateway API.
//!
//! This module provides functionalities to create, list, and delete `TLSRoute` resources.
//! It supports advanced configuration for SNI hosts and backend references.

use kube::{Api, Client, CustomResourceExt};
use kube::api::{ApiResource, DeleteParams, ListParams, PostParams, DynamicObject};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube_derive::CustomResource;
use kube::core::GroupVersionKind;
use thiserror::Error;

/// `TLSRouteInnerSpec` defines the internal specification for a TLSRoute resource.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TLSRouteInnerSpec {
    pub sni_hosts: Vec<String>,           // SNI hosts for this TLS route
    pub backend_refs: Vec<BackendReference>, // References to backend services
}

/// `BackendReference` specifies a backend target for the TLS route.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BackendReference {
    pub name: String,                     // Name of the backend
    pub port: u16,                        // Port of the backend
}

/// Define the custom resource `TLSRoute` using the `CustomResource` macro.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "networking.k8s.io",          // Group for the custom resource
    version = "v1alpha1",                 // API version
    kind = "TLSRoute",                    // Resource kind
    namespaced                            // Namespaced resource
)]
pub struct TLSRouteSpec {
    pub spec: TLSRouteInnerSpec,
}

/// Manages the lifecycle of `TLSRoute` resources in Kubernetes.
pub struct TLSRouteManager {
    client: Client,                       // Kubernetes client
    ar: ApiResource,                      // API resource details for `TLSRoute`
}

impl TLSRouteManager {
    /// Creates a new `TLSRouteManager`.
    pub fn new(client: Client) -> Self {
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "TLSRoute");
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
    }

    /// Lists all `TLSRoute` resources in the specified namespace.
    pub async fn list_tls_routes(&self, namespace: &str) -> Result<Vec<DynamicObject>, TLSRouteError> {
        let tls_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let lp = ListParams::default();
        match tls_routes.list(&lp).await {
            Ok(route_list) => Ok(route_list.items),
            Err(e) => Err(TLSRouteError::KubeError(e)),
        }
    }

    /// Creates a new `TLSRoute` resource in the specified namespace.
    pub async fn create_tls_route(
        &self,
        namespace: &str,
        tls_route: &TLSRouteSpec,
    ) -> Result<DynamicObject, TLSRouteError> {
        let tls_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let pp = PostParams::default();

        // Set metadata with a name derived from the first SNI host
        let name = tls_route.spec.sni_hosts.get(0)
            .map(|host| host.replace(".", "-")) // Replace dots to create a valid name
            .unwrap_or_else(|| "default-tlsroute".to_string());

        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(tls_route)?.as_object().cloned().unwrap_or_default(),
        );

        match tls_routes.create(&pp, &crd).await {
            Ok(created) => Ok(created),
            Err(e) => Err(TLSRouteError::KubeError(e)),
        }
    }

    /// Deletes a `TLSRoute` resource by name from the specified namespace.
    pub async fn delete_tls_route(&self, namespace: &str, name: &str) -> Result<(), TLSRouteError> {
        let tls_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        match tls_routes.delete(name, &DeleteParams::default()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(TLSRouteError::KubeError(e)),
        }
    }
}

/// Error types for `TLSRouteManager` operations.
#[derive(Error, Debug)]
pub enum TLSRouteError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),       // Errors from the Kubernetes API
    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error), // Errors in JSON serialization/deserialization
}

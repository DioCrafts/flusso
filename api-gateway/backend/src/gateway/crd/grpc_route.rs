//! Module for managing GRPCRoute resources in Kubernetes using the Gateway API.

use kube::{Api, Client, CustomResourceExt};
use kube::api::{DynamicObject, ApiResource, ListParams, PostParams, DeleteParams};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube_derive::CustomResource;
use thiserror::Error;
use kube::core::GroupVersionKind;

/// Define the inner specification for a GRPCRoute.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GRPCRouteInnerSpec {
    pub match_criteria: String,              // Match criteria for gRPC traffic.
    pub backend_refs: Vec<BackendReference>, // Target backends.
}

/// Define a backend reference structure.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BackendReference {
    pub name: String, // Name of the backend service.
    pub port: u16,    // Port for the backend service.
}

/// Define the GRPCRoute Custom Resource.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "networking.k8s.io",  // Group name for the CRD.
    version = "v1alpha1",         // Version of the CRD.
    kind = "GRPCRoute",           // Kind of the resource.
    namespaced                   // Indicates this resource is namespaced.
)]
pub struct GRPCRouteSpec {
    pub spec: GRPCRouteInnerSpec, // Specification for the gRPC route.
}

/// A manager for handling GRPCRoute resources.
pub struct GRPCRouteManager {
    client: Client,
    ar: ApiResource, // Defines the resource details for the GRPCRoute.
}

impl GRPCRouteManager {
    /// Create a new GRPCRouteManager instance.
    pub fn new(client: Client) -> Self {
        // Define the ApiResource for the GRPCRoute CRD.
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "GRPCRoute");
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
    }

    /// List all GRPCRoute resources in a namespace.
    pub async fn list_grpc_routes(
        &self,
        namespace: &str,
    ) -> Result<Vec<DynamicObject>, GRPCRouteError> {
        let grpc_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let list_params = ListParams::default();
        match grpc_routes.list(&list_params).await {
            Ok(route_list) => Ok(route_list.items),
            Err(e) => Err(GRPCRouteError::KubeError(e)),
        }
    }

    /// Create a new GRPCRoute in a namespace.
    pub async fn create_grpc_route(
        &self,
        namespace: &str,
        grpc_route: &GRPCRouteSpec,
    ) -> Result<DynamicObject, GRPCRouteError> {
        let grpc_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let post_params = PostParams::default();

        // Use the match_criteria as the name for the resource.
        let name = grpc_route.spec.match_criteria.clone();

        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(grpc_route)?.as_object().cloned().unwrap_or_default(),
        );

        match grpc_routes.create(&post_params, &crd).await {
            Ok(created_route) => Ok(created_route),
            Err(e) => Err(GRPCRouteError::KubeError(e)),
        }
    }

    /// Delete a GRPCRoute by name from a namespace.
    pub async fn delete_grpc_route(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<(), GRPCRouteError> {
        let grpc_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        match grpc_routes.delete(name, &DeleteParams::default()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(GRPCRouteError::KubeError(e)),
        }
    }
}

/// Define custom error types for GRPCRoute operations.
#[derive(Error, Debug)]
pub enum GRPCRouteError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

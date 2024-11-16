use kube::{Api, Client, CustomResourceExt};
use kube::api::{DynamicObject, ApiResource, ListParams, PostParams, DeleteParams};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube_derive::CustomResource;
use thiserror::Error;
use kube::core::GroupVersionKind;

/// Define `GRPCRouteInnerSpec` for specifying gRPC traffic routes and parameters.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GRPCRouteInnerSpec {
    pub match_criteria: String, // Match criteria for gRPC traffic
    pub backend_refs: Vec<BackendReference>, // Target backends
}

/// Define `BackendReference` for specifying backend name and port.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BackendReference {
    pub name: String,
    pub port: u16,
}

/// Define the CRD `GRPCRoute` as a `CustomResource` for the gRPC route API.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "networking.flusso.io",
    version = "v1alpha1",
    kind = "GRPCRoute",
    namespaced
)]
pub struct GRPCRouteSpec {
    pub spec: GRPCRouteInnerSpec,
}

/// `GRPCRouteManager` manages the lifecycle of `GRPCRoute` resources.
pub struct GRPCRouteManager {
    client: Client,
    ar: ApiResource, // Defines the resource details for DynamicObject
}

impl GRPCRouteManager {
    pub fn new(client: Client) -> Self {
        // Crea un GroupVersionKind con el grupo, versión y tipo del recurso
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "Gateway");

        // Define el ApiResource para el CRD personalizado Gateway
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
    }

    /// List all `GRPCRoute` resources across all namespaces.
    pub async fn list_grpc_routes(&self, namespace: &str) -> Result<Vec<DynamicObject>, GRPCRouteError> {
        let grpc_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let list_params = ListParams::default();
        let grpc_route_list = grpc_routes.list(&list_params).await?;
        Ok(grpc_route_list.items)
    }

    /// Create a new `GRPCRoute` resource in the specified namespace.
    pub async fn create_grpc_route(
        &self,
        namespace: &str,
        grpc_route: &GRPCRouteSpec,
    ) -> Result<DynamicObject, GRPCRouteError> {
        let grpc_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let post_params = PostParams::default();

        // Set metadata with a default name if not present
        let name = grpc_route.spec.match_criteria.clone(); // Adjust as needed

        // Create a new DynamicObject
        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(grpc_route)?.as_object().cloned().unwrap_or_default()
        );

        // Send the request to create the resource
        let created_grpc_route = grpc_routes.create(&post_params, &crd).await?;
        Ok(created_grpc_route)
    }

    /// Delete a `GRPCRoute` resource by name from the specified namespace.
    pub async fn delete_grpc_route(&self, namespace: &str, name: &str) -> Result<(), GRPCRouteError> {
        let grpc_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        grpc_routes.delete(name, &DeleteParams::default()).await?;
        Ok(())
    }
}

/// Define the error type for `GRPCRoute` operations.
#[derive(Error, Debug)]
pub enum GRPCRouteError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Failed to parse GRPCRoute configuration: {0}")]
    ParseError(#[from] serde_json::Error),
}

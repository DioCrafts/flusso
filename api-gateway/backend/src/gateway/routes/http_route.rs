//! src/gateway/routes/http_route.rs
use kube::{Api, Client, CustomResourceExt};
use kube::api::{DynamicObject, ApiResource, ListParams, PostParams, Patch, PatchParams, DeleteParams};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube_derive::CustomResource;
use thiserror::Error;
use kube::core::GroupVersionKind;


/// `HTTPRouteInnerSpec` defines the specification for an HTTPRoute resource.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteInnerSpec {
    pub hostnames: Option<Vec<String>>,
    pub rules: Vec<HTTPRouteRule>,
}

/// `HTTPRouteRule` defines a single routing rule for an HTTPRoute.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteRule {
    pub matches: Option<Vec<HTTPRouteMatch>>,
    pub filters: Option<Vec<HTTPRouteFilter>>,
    pub backend_refs: Vec<BackendReference>,
}

/// `HTTPRouteMatch` specifies how to match HTTP requests.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteMatch {
    pub path: Option<HTTPPathMatch>,
    pub headers: Option<Vec<HTTPHeaderMatch>>,
}

/// `HTTPPathMatch` specifies a path match rule.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPPathMatch {
    pub type_: Option<String>,
    pub value: Option<String>,
}

/// `HTTPHeaderMatch` specifies a header match rule.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPHeaderMatch {
    pub name: String,
    pub value: String,
}

/// `HTTPRouteFilter` specifies a filter to apply to HTTP requests or responses.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteFilter {
    pub type_: String,
    pub value: Option<String>,
}

/// `BackendReference` specifies the backend to route to.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BackendReference {
    pub name: String,
    pub port: Option<u16>,
}

/// Define `HTTPRoute` as a custom resource for managing HTTP routing.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "networking.k8s.io",
    version = "v1alpha1",
    kind = "HTTPRoute",
    namespaced
)]
pub struct HTTPRouteSpec {
    pub spec: HTTPRouteInnerSpec,
}

/// `HTTPRouteManager` manages the lifecycle of `HTTPRoute` resources.
pub struct HTTPRouteManager {
    client: Client,
    ar: ApiResource, // Defines the resource details for DynamicObject
}

impl HTTPRouteManager {
    pub fn new(client: Client) -> Self {
        // Crea un GroupVersionKind con el grupo, versión y tipo del recurso
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "Gateway");

        // Define el ApiResource para el CRD personalizado Gateway
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
    }    

    /// List all `HTTPRoute` resources in the specified namespace.
    pub async fn list_http_routes(&self, namespace: &str) -> Result<Vec<DynamicObject>, HTTPRouteError> {
        let http_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let lp = ListParams::default();
        let http_route_list = http_routes.list(&lp).await?;
        Ok(http_route_list.items)
    }

    /// Create a new `HTTPRoute` resource in the specified namespace.
    pub async fn create_http_route(
        &self,
        namespace: &str,
        http_route: &HTTPRouteSpec,
    ) -> Result<DynamicObject, HTTPRouteError> {
        let http_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let pp = PostParams::default();

        let name = http_route.spec.hostnames.clone().unwrap_or_default().get(0).unwrap_or(&"default-host".to_string()).clone();

        // Create a new DynamicObject
        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(http_route)?.as_object().cloned().unwrap_or_default()
        );

        let created_http_route = http_routes.create(&pp, &crd).await?;
        Ok(created_http_route)
    }

    /// Update an existing `HTTPRoute` resource in the specified namespace.
    pub async fn update_http_route(
        &self,
        namespace: &str,
        name: &str,
        http_route: &HTTPRouteSpec,
    ) -> Result<DynamicObject, HTTPRouteError> {
        let http_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let patch_params = PatchParams::apply("flusso");

        let mut crd = DynamicObject::new(name, &self.ar);
        crd.data = serde_json::Value::Object(serde_json::to_value(http_route)?.as_object().cloned().unwrap_or_default());

        let updated_http_route = http_routes.patch(name, &patch_params, &Patch::Apply(&crd)).await?;
        Ok(updated_http_route)
    }

    /// Delete a `HTTPRoute` resource by name from the specified namespace.
    pub async fn delete_http_route(&self, namespace: &str, name: &str) -> Result<(), HTTPRouteError> {
        let http_routes: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        http_routes.delete(name, &DeleteParams::default()).await?;
        Ok(())
    }
}

/// Define the error type for `HTTPRoute` operations.
#[derive(Error, Debug)]
pub enum HTTPRouteError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

//! src/gateway/crd/http_route.rs
//!
//! Módulo para gestionar recursos HTTPRoute en Kubernetes utilizando la Gateway API.
//! Este módulo proporciona funcionalidades para listar, crear, actualizar y eliminar rutas HTTP.

use kube::{Api, Client, CustomResourceExt};
use kube::api::{ListParams, PostParams, DeleteParams, Patch, PatchParams, DynamicObject, ApiResource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kube_derive::CustomResource;
use thiserror::Error;
use kube::core::GroupVersionKind;

/// Especificación de la ruta HTTP (`HTTPRouteSpec`).
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteInnerSpec {
    pub hostnames: Option<Vec<String>>, // Nombres de host soportados por la ruta
    pub rules: Vec<HTTPRouteRule>,      // Reglas asociadas a la ruta HTTP
}

/// Especificación de una regla dentro de la ruta HTTP (`HTTPRouteRule`).
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteRule {
    pub matches: Option<Vec<HTTPRouteMatch>>,  // Condiciones para coincidir con solicitudes
    pub filters: Option<Vec<HTTPRouteFilter>>, // Filtros aplicados a las solicitudes
    pub backend_refs: Vec<BackendReference>,  // Backends a los que se debe enrutar
}

/// Especificación de coincidencias en solicitudes HTTP (`HTTPRouteMatch`).
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteMatch {
    pub path: Option<HTTPPathMatch>, // Coincidencia basada en el path
    pub headers: Option<Vec<HTTPHeaderMatch>>, // Coincidencia basada en headers
}

/// Coincidencia de path en solicitudes HTTP (`HTTPPathMatch`).
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPPathMatch {
    pub type_: Option<String>, // Tipo de coincidencia (Exact, Prefix, Regex)
    pub value: Option<String>, // Valor del path a coincidir
}

/// Coincidencia de headers en solicitudes HTTP (`HTTPHeaderMatch`).
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPHeaderMatch {
    pub name: String,  // Nombre del header
    pub value: String, // Valor del header
}

/// Filtro aplicado a las solicitudes o respuestas (`HTTPRouteFilter`).
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HTTPRouteFilter {
    pub type_: String,  // Tipo de filtro (RequestHeaderModifier, ResponseHeaderModifier, etc.)
    pub value: Option<String>, // Valor asociado al filtro
}

/// Referencia al backend al que se debe enrutar (`BackendReference`).
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BackendReference {
    pub name: String,      // Nombre del backend
    pub port: Option<u16>, // Puerto del backend
}

/// Definición del recurso `HTTPRoute` como CRD.
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

/// Gestor para operaciones con recursos `HTTPRoute`.
pub struct HTTPRouteManager {
    client: Client,
    ar: ApiResource, // Define los detalles del recurso como DynamicObject
}

impl HTTPRouteManager {
    /// Crear una nueva instancia del gestor de rutas HTTP.
    pub fn new(client: Client) -> Self {
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "HTTPRoute");
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
    }

    /// Listar todos los recursos `HTTPRoute` en un namespace.
    pub async fn list_http_routes(
        &self,
        namespace: &str,
    ) -> Result<Vec<DynamicObject>, HTTPRouteError> {
        let api: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let list_params = ListParams::default();
        let route_list = api.list(&list_params).await?;
        Ok(route_list.items)
    }

    /// Crear un nuevo recurso `HTTPRoute`.
    pub async fn create_http_route(
        &self,
        namespace: &str,
        http_route: &HTTPRouteSpec,
    ) -> Result<DynamicObject, HTTPRouteError> {
        let api: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let post_params = PostParams::default();

        // Generar DynamicObject basado en HTTPRouteSpec
        let name = http_route.spec.hostnames.clone().unwrap_or_default().get(0).unwrap_or(&"default".to_string()).clone();
        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(http_route)?.as_object().cloned().unwrap_or_default(),
        );

        let created_route = api.create(&post_params, &crd).await?;
        Ok(created_route)
    }

    /// Actualizar un recurso `HTTPRoute` existente.
    pub async fn update_http_route(
        &self,
        namespace: &str,
        name: &str,
        http_route: &HTTPRouteSpec,
    ) -> Result<DynamicObject, HTTPRouteError> {
        let api: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let patch_params = PatchParams::apply("flusso");

        let mut crd = DynamicObject::new(name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(http_route)?.as_object().cloned().unwrap_or_default(),
        );

        let updated_route = api.patch(name, &patch_params, &Patch::Apply(&crd)).await?;
        Ok(updated_route)
    }

    /// Eliminar un recurso `HTTPRoute` por su nombre.
    pub async fn delete_http_route(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<(), HTTPRouteError> {
        let api: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        api.delete(name, &DeleteParams::default()).await?;
        Ok(())
    }
}

/// Error para operaciones con recursos `HTTPRoute`.
#[derive(Error, Debug)]
pub enum HTTPRouteError {
    #[error("Error de Kubernetes API: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Error en serialización/deserialización: {0}")]
    SerdeError(#[from] serde_json::Error),
}

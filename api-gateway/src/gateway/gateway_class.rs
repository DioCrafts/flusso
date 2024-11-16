// src/gateway/gateway_class.rs

//! Module for managing GatewayClass resources in Kubernetes using the Gateway API.
//!
//! This module provides structs and functions to create, list, and manage GatewayClass resources,
//! which define the types of gateways that can be instantiated in the cluster.

use kube::{Api, Client, CustomResourceExt};
use kube::api::{DynamicObject, ApiResource, ListParams, PostParams, DeleteParams};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube_derive::CustomResource;
use thiserror::Error;
use kube::core::GroupVersionKind;


/// GatewayClassInnerSpec defines the specification for a GatewayClass resource.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GatewayClassInnerSpec {
    pub controller_name: String,
    pub parameters_ref: Option<ParametersReference>,
}

/// ParametersReference is an optional reference to a custom configuration for the GatewayClass.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ParametersReference {
    pub api_group: Option<String>,
    pub kind: String,
    pub name: String,
    pub namespace: Option<String>,
}

/// Define the `GatewayClass` CRD for the Gateway API.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "networking.k8s.io",
    version = "v1alpha1",
    kind = "GatewayClass",
    namespaced
)]
pub struct GatewayClassSpec {
    pub spec: GatewayClassInnerSpec,
}

/// GatewayClassManager is responsible for interacting with `GatewayClass` resources in Kubernetes.
pub struct GatewayClassManager {
    client: Client,
    ar: ApiResource, // ApiResource defining resource details for DynamicObject
}

impl GatewayClassManager {
    pub fn new(client: Client) -> Self {
        // Crea un GroupVersionKind con el grupo, versión y tipo del recurso
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "Gateway");

        // Define el ApiResource para el CRD personalizado Gateway
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
    }

    /// List all GatewayClasses in the specified namespace.
    pub async fn list_gateway_classes(&self) -> Result<Vec<DynamicObject>, GatewayClassError> {
        let gateway_classes: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        let lp = ListParams::default();
        let gateway_class_list = gateway_classes.list(&lp).await?;
        Ok(gateway_class_list.items)
    }

    /// Create a new GatewayClass resource.
    pub async fn create_gateway_class(
        &self,
        gateway_class: &GatewayClassSpec,
    ) -> Result<DynamicObject, GatewayClassError> {
        let gateway_classes: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        let pp = PostParams::default();

        let name = gateway_class.spec.controller_name.clone();

        // Create a new DynamicObject
        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(gateway_class)?.as_object().cloned().unwrap_or_default()
        );

        let created_gateway_class = gateway_classes.create(&pp, &crd).await?;
        Ok(created_gateway_class)
    }

    /// Retrieve a GatewayClass by name.
    pub async fn get_gateway_class(&self, name: &str) -> Result<Option<DynamicObject>, GatewayClassError> {
        let gateway_classes: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        match gateway_classes.get(name).await {
            Ok(gateway_class) => Ok(Some(gateway_class)),
            Err(kube::Error::Api(err)) if err.code == 404 => Ok(None),
            Err(e) => Err(GatewayClassError::KubeError(e)),
        }
    }

    /// Delete a GatewayClass by name.
    pub async fn delete_gateway_class(&self, name: &str) -> Result<(), GatewayClassError> {
        let gateway_classes: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        gateway_classes.delete(name, &DeleteParams::default()).await?;
        Ok(())
    }
}

/// Define the error type for `GatewayClass` operations.
#[derive(Error, Debug)]
pub enum GatewayClassError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

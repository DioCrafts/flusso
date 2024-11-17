// src/gateway/gateway.rs

//! Module for managing Gateway resources in Kubernetes using the Gateway API as a Custom Resource.

use kube::{Api, Client, CustomResourceExt};
use kube::api::{DynamicObject, ApiResource, ListParams, PostParams};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kube_derive::CustomResource;
use thiserror::Error;
use kube::api::DeleteParams;
use kube::core::GroupVersionKind;


/// GatewayInnerSpec defines the internal specification for a Gateway resource.
/// It includes the gateway's class name and a list of listeners.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GatewayInnerSpec {
    pub gateway_class_name: String, // Name of the gateway class
    pub listeners: Vec<Listener>,   // List of listeners defined for the gateway
}

/// Listener defines the configuration for each listener in a Gateway,
/// including name, protocol, and port.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Listener {
    pub name: String,      // Listener name
    pub protocol: String,  // Protocol (e.g., HTTP, HTTPS)
    pub port: u16,         // Port on which the listener operates
}

/// Status struct for GatewayStatus.
/// This struct can be extended to reflect the status of the Gateway resource in Kubernetes.
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct GatewayStatus {}

/// Define the custom Gateway resource (CRD) for the Gateway API in Kubernetes.
/// This struct represents the schema of the Gateway resource and is serializable/deserializable.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "networking.k8s.io",        // Group the CRD belongs to
    version = "v1alpha1",               // API version of the resource
    kind = "Gateway",                   // Kind of resource (CRD type)
    namespaced,                         // Specifies that the resource is namespaced
    status = "GatewayStatus"            // Status struct for the resource
)]
pub struct GatewaySpec {
    pub spec: GatewayInnerSpec, // Main specification of the Gateway resource
}

/// `GatewayManager` is a struct that facilitates the management of Gateway resources in Kubernetes.
/// It includes functions to list, create, and retrieve Gateway resources using a Kubernetes client.
pub struct GatewayManager {
    client: Client,           // Kubernetes client
    ar: ApiResource,          // ApiResource defining details for the DynamicObject
}

impl GatewayManager {
    /// Creates a new instance of `GatewayManager` with the specified Kubernetes client.
    pub fn new(client: Client) -> Self {
        // Crea un GroupVersionKind con el grupo, versión y tipo del recurso
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "Gateway");
    
        // Define el ApiResource para el CRD personalizado Gateway
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
       
    }

    /// Lists all Gateway resources in the specified namespace.
    ///
    /// # Arguments
    /// - `namespace`: Name of the namespace to list resources in.
    ///
    /// # Returns
    /// A vector of `DynamicObject` containing the listed Gateway resources.
    pub async fn list_gateways(&self, namespace: &str) -> Result<Vec<DynamicObject>, GatewayError> {
        // Creates an API client for the DynamicObject resource within the specified namespace
        let gateways: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let lp = ListParams::default();
        let gateway_list = gateways.list(&lp).await?;
        Ok(gateway_list.items)
    }

    /// Creates a new Gateway resource in the specified namespace.
    ///
    /// # Arguments
    /// - `namespace`: Name of the namespace where the resource will be created.
    /// - `gateway`: Reference to the `GatewaySpec` object defining the resource to create.
    ///
    /// # Returns
    /// The `DynamicObject` created as a result of the operation.
    pub async fn create_gateway(
        &self,
        namespace: &str,
        gateway: &GatewaySpec,
    ) -> Result<DynamicObject, GatewayError> {
        let gateways: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let pp = PostParams::default();

        // Set metadata with a default name if not present
        let name = gateway.spec.gateway_class_name.clone(); // Using class name as the identifier; adjust as needed

        // Create a new DynamicObject with Gateway resource data
        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(gateway)?.as_object().cloned().unwrap_or_default()
        );

        // Send the request to create the resource
        let created_gateway = gateways.create(&pp, &crd).await?;
        Ok(created_gateway)
    }

    /// Retrieves a Gateway resource by name in the specified namespace.
    ///
    /// # Arguments
    /// - `namespace`: Name of the namespace to search for the resource.
    /// - `name`: Name of the Gateway resource to retrieve.
    ///
    /// # Returns
    /// An `Option<DynamicObject>` containing the Gateway resource if found, or `None` if not found.
    pub async fn get_gateway(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<Option<DynamicObject>, GatewayError> {
        let gateways: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        match gateways.get(name).await {
            Ok(gateway) => Ok(Some(gateway)),
            Err(kube::Error::Api(err)) if err.code == 404 => Ok(None),
            Err(e) => Err(GatewayError::KubeError(e)),
        }
    }

    /// Deletes a Gateway resource by name in the specified namespace.
    pub async fn delete_gateway(&self, namespace: &str, name: &str) -> Result<(), GatewayError> {
        let gateways: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        match gateways.delete(name, &DeleteParams::default()).await {
            Ok(_) => Ok(()),
            Err(kube::Error::Api(err)) if err.code == 404 => Ok(()), // Return Ok if the resource is not found
            Err(e) => Err(GatewayError::KubeError(e)),
        }
    }

}

/// Define the error type for Gateway-related operations.
#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),               // Error related to the Kubernetes API
    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),        // Error in serialization or deserialization
}

//! src/gateway/crd/gateway_class.rs
//!
//! Gestión de GatewayClass en Kubernetes.
//!
//! Este módulo proporciona funciones para listar, crear, actualizar y eliminar
//! recursos de tipo GatewayClass utilizando el cliente de Kubernetes.

use kube::{Api, Client};
use kube::api::{ApiResource, DynamicObject, ListParams, PostParams, DeleteParams};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube_derive::CustomResource;
use kube::core::GroupVersionKind;
use thiserror::Error;

/// Especificación interna de GatewayClass.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GatewayClassInnerSpec {
    pub controller_name: String,            // Nombre del controlador asociado
    pub parameters_ref: Option<ParametersReference>, // Referencia opcional a parámetros adicionales
}

/// Referencia a parámetros opcionales para la configuración del GatewayClass.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ParametersReference {
    pub api_group: Option<String>, // Grupo del API
    pub kind: String,              // Tipo de recurso
    pub name: String,              // Nombre del recurso
    pub namespace: Option<String>, // Namespace (si es necesario)
}

/// Definición del recurso personalizado GatewayClass como CRD.
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

/// Manager para gestionar GatewayClass en Kubernetes.
pub struct GatewayClassManager {
    client: Client,
    ar: ApiResource, // Define el recurso para DynamicObject
}

impl GatewayClassManager {
    /// Crea una nueva instancia del manager.
    pub fn new(client: Client) -> Self {
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "GatewayClass");
        let ar = ApiResource::from_gvk(&gvk);
        Self { client, ar }
    }

    /// Lista todos los GatewayClass disponibles en el clúster.
    pub async fn list_gateway_classes(&self) -> Result<Vec<DynamicObject>, GatewayClassError> {
        let gateway_classes: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        let lp = ListParams::default();
        let gateway_class_list = gateway_classes.list(&lp).await?;
        Ok(gateway_class_list.items)
    }

    /// Crea un nuevo GatewayClass en el clúster.
    pub async fn create_gateway_class(
        &self,
        gateway_class: &GatewayClassSpec,
    ) -> Result<DynamicObject, GatewayClassError> {
        let gateway_classes: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        let pp = PostParams::default();

        let name = gateway_class.spec.controller_name.clone();

        // Crear un DynamicObject
        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(gateway_class)?.as_object().cloned().unwrap_or_default(),
        );

        let created_gateway_class = gateway_classes.create(&pp, &crd).await?;
        Ok(created_gateway_class)
    }

    /// Obtiene un GatewayClass específico por su nombre.
    pub async fn get_gateway_class(&self, name: &str) -> Result<Option<DynamicObject>, GatewayClassError> {
        let gateway_classes: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        match gateway_classes.get(name).await {
            Ok(gateway_class) => Ok(Some(gateway_class)),
            Err(kube::Error::Api(err)) if err.code == 404 => Ok(None),
            Err(e) => Err(GatewayClassError::KubeError(e)),
        }
    }

    /// Elimina un GatewayClass por su nombre.
    pub async fn delete_gateway_class(&self, name: &str) -> Result<(), GatewayClassError> {
        let gateway_classes: Api<DynamicObject> = Api::all_with(self.client.clone(), &self.ar);
        gateway_classes.delete(name, &DeleteParams::default()).await?;
        Ok(())
    }
}

/// Definición de errores específicos para GatewayClass.
#[derive(Error, Debug)]
pub enum GatewayClassError {
    #[error("Error en la API de Kubernetes: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Error de serialización/deserialización: {0}")]
    SerdeError(#[from] serde_json::Error),
}

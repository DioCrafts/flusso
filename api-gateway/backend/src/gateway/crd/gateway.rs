//! src/gateway/crd/gateway.rs
//!
//! Gestión de los recursos Gateway en Kubernetes.
//!
//! Este módulo implementa el `GatewayManager`, que proporciona métodos para
//! crear, listar, eliminar y configurar Gateways como CRDs.

use kube::{Api, Client, CustomResourceExt};
use kube::api::{PostParams, DeleteParams, ListParams, ApiResource, DynamicObject};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube_derive::CustomResource;
use thiserror::Error;
use kube::core::GroupVersionKind;

/// Especificación interna para el recurso Gateway.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GatewayInnerSpec {
    pub hostname: String,           // Nombre del host
    pub tls_enabled: bool,          // TLS habilitado o no
    pub certificate: Option<String>, // Certificado TLS (si aplica)
    pub routes: Vec<Route>,         // Lista de rutas asociadas al Gateway
}

/// Definición de rutas asociadas al Gateway.
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub path: String,         // Path como "/api/v1/*" o expresiones regulares
    pub backend: String,      // Backend al que se enruta
    pub methods: Vec<String>, // Métodos HTTP permitidos (GET, POST, etc.)
}

/// Definición del CRD `Gateway`.
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "networking.k8s.io",    // Grupo del CRD
    version = "v1alpha1",           // Versión del API
    kind = "Gateway",               // Tipo de recurso
    namespaced,                     // Es un recurso namespaced
)]
pub struct GatewaySpec {
    pub spec: GatewayInnerSpec, // Especificación principal del Gateway
}

/// Manager para interactuar con los recursos Gateway en Kubernetes.
pub struct GatewayManager {
    client: Client,           // Cliente de Kubernetes
    ar: ApiResource,          // ApiResource que define los detalles del recurso DynamicObject
}

impl GatewayManager {
    /// Crear una nueva instancia del `GatewayManager`.
    pub fn new(client: Client) -> Self {
        // Definir el ApiResource para el recurso Gateway.
        let gvk = GroupVersionKind::gvk("networking.k8s.io", "v1alpha1", "Gateway");
        let ar = ApiResource::from_gvk(&gvk);

        Self { client, ar }
    }

    /// Listar todos los Gateways en el namespace especificado.
    pub async fn list_gateways(&self, namespace: &str) -> Result<Vec<DynamicObject>, GatewayError> {
        let gateways: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let lp = ListParams::default();
        let gateway_list = gateways.list(&lp).await?;
        Ok(gateway_list.items)
    }

    /// Crear un nuevo Gateway.
    pub async fn create_gateway(
        &self,
        namespace: &str,
        gateway: &GatewaySpec,
    ) -> Result<DynamicObject, GatewayError> {
        let gateways: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        let pp = PostParams::default();

        // Usar el hostname como nombre del recurso.
        let name = gateway.spec.hostname.clone();

        // Crear un DynamicObject con la información del Gateway.
        let mut crd = DynamicObject::new(&name, &self.ar);
        crd.data = serde_json::Value::Object(
            serde_json::to_value(gateway)?.as_object().cloned().unwrap_or_default()
        );

        let created_gateway = gateways.create(&pp, &crd).await?;
        Ok(created_gateway)
    }

    /// Eliminar un Gateway por nombre.
    pub async fn delete_gateway(&self, namespace: &str, name: &str) -> Result<(), GatewayError> {
        let gateways: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
        gateways.delete(name, &DeleteParams::default()).await?;
        Ok(())
    }

    /// Configurar TLS para un Gateway.
    pub async fn configure_tls(
        &self,
        namespace: &str,
        name: &str,
        certificate: &str,
    ) -> Result<(), GatewayError> {
        let gateways: Api<DynamicObject> = Api::namespaced_with(self.client.clone(), namespace, &self.ar);
    
        // Obtener el Gateway existente.
        let mut existing_gateway = gateways.get(name).await?;
        let mut data = existing_gateway.data.clone(); // Clonar el campo data
    
        // Actualizar el certificado y habilitar TLS.
        if let Some(spec) = data.get_mut("spec") {
            if let Some(spec_obj) = spec.as_object_mut() {
                spec_obj.insert("tlsEnabled".to_string(), serde_json::Value::Bool(true));
                spec_obj.insert("certificate".to_string(), serde_json::Value::String(certificate.to_string()));
            }
        }
    
        // Reemplazar el Gateway con los cambios actualizados.
        existing_gateway.data = data; // Actualizar el campo data después de la edición
        gateways.replace(name, &PostParams::default(), &existing_gateway).await?;
        Ok(())
    }
    
    /// Obtener métricas simuladas de los Gateways.
    pub async fn get_metrics(&self) -> Result<Vec<(String, String, u64)>, GatewayError> {
        // Ejemplo de métricas simuladas.
        Ok(vec![
            ("gateway_1".to_string(), "latency".to_string(), 120),
            ("gateway_1".to_string(), "requests".to_string(), 1500),
        ])
    }
}

/// Definición de errores específicos para los Gateways.
#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Error en la API de Kubernetes: {0}")]
    KubeError(#[from] kube::Error),
    #[error("Error de serialización/deserialización: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Implementación de conversión de `models::Route` a `gateway::crd::gateway::Route`.
impl From<crate::gateway::models::Route> for Route {
    fn from(route: crate::gateway::models::Route) -> Self {
        Route {
            path: route.path.unwrap_or_default(),
            backend: route.backend,
            methods: route.methods,
        }
    }
}

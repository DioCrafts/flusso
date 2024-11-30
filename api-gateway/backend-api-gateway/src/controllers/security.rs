// src/controllers/security.rs
use kube::{
    api::{Api, Patch, PatchParams, Resource},
    Client, CustomResource,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "gateway.flusso.io",
    version = "v1alpha1",
    kind = "SecurityConfig",
    namespaced
)]
pub struct SecurityConfigSpec {
    pub rate_limit: RateLimitConfig,
    pub authentication: AuthConfig,
    pub cors: CorsConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_minute: i32,
    pub burst_size: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiration: i32,
    pub refresh_token_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct CorsConfig {
    pub enabled: bool,
    pub allowed_origins: String,
    pub allowed_methods: Vec<String>,
}

pub async fn apply_security_config(
    client: Client,
    namespace: &str,
    config: SecurityConfigSpec,
) -> Result<(), kube::Error> {
    let configs: Api<SecurityConfig> = Api::namespaced(client, namespace);
    
    let config_name = "api-gateway-security";
    let patch = Patch::Apply(json!({
        "apiVersion": "gateway.flusso.io/v1alpha1",
        "kind": "SecurityConfig",
        "spec": config
    }));

    let patch_params = PatchParams::apply("api-gateway-controller");
    configs.patch(config_name, &patch_params, &patch).await?;

    Ok(())
}
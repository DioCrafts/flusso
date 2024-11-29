// src/config/mod.rs
mod routes;
mod settings;

pub use routes::{Route, RouteConfig};
pub use settings::{Settings, ServerSettings, DatabaseSettings, JwtSettings};

use anyhow::Result;

pub fn load_configuration() -> Result<Settings> {
    let settings = Settings::new()?;
    Ok(settings)
}
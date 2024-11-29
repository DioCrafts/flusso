// src/middleware/mod.rs
pub mod auth;
pub mod logging;
pub mod metrics;

use actix_web::Error;
use std::future::Future;
use std::pin::Pin;

pub type MiddlewareResult = Result<(), Error>;
pub type MiddlewareFuture = Pin<Box<dyn Future<Output = MiddlewareResult>>>;

// src/middleware/logging.rs
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    time::Instant,
};
use tracing::{info, error};

pub struct Logger;

impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggerMiddleware { service }))
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => {
                    let duration = start_time.elapsed();
                    info!(
                        method = %method,
                        path = %path,
                        status = %res.status().as_u16(),
                        duration = ?duration,
                        "Request completed"
                    );
                    Ok(res)
                }
                Err(err) => {
                    error!(
                        method = %method,
                        path = %path,
                        error = %err,
                        "Request failed"
                    );
                    Err(err)
                }
            }
        })
    }
}
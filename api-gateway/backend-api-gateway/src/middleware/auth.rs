// src/middleware/auth.rs
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use crate::handlers::auth::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub struct Auth {
    jwt_secret: Rc<String>,
}

impl Auth {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret: Rc::new(jwt_secret),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static, // Añadimos Clone aquí
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    jwt_secret: Rc<String>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static, // Añadimos la restricción 'static
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone(); // Clonamos el jwt_secret
        let service = self.service.clone(); // Clonamos el servicio subyacente

        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                let auth_str = auth_header.to_str().map_err(|_| {
                    actix_web::error::ErrorUnauthorized("Invalid authorization header")
                })?;

                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(jwt_secret.as_bytes()),
                        &Validation::default(),
                    ) {
                        Ok(token_data) => {
                            req.extensions_mut().insert(token_data.claims);
                            // Llamar al servicio subyacente
                            service.call(req).await
                        }
                        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
                    }
                } else {
                    Err(actix_web::error::ErrorUnauthorized("Invalid token format"))
                }
            } else {
                Err(actix_web::error::ErrorUnauthorized("Missing authorization"))
            }
        })
    }
}

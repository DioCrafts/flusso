// src/handlers/auth.rs
use actix_web::{web, HttpResponse, HttpRequest};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::config::Settings;
use chrono::{Utc, Duration};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub role: String,
}

pub struct AuthHandler {
    config: Arc<Settings>,
}

impl AuthHandler {
    pub fn new(config: Arc<Settings>) -> Self {
        Self { config }
    }


    pub async fn login(
        auth_handler: web::Data<AuthHandler>,
        login_req: web::Json<LoginRequest>,
    ) -> Result<HttpResponse, actix_web::Error> {
        // Validación de credenciales
        if login_req.email == "admin@test.com" && login_req.password == "admin123" {
            let expiration = Utc::now()
                .checked_add_signed(Duration::seconds(
                    auth_handler.config.jwt.expiration as i64,
                ))
                .expect("Error calculando expiración")
                .timestamp();
    
            let claims = Claims {
                sub: login_req.email.clone(),
                exp: expiration,
                role: "admin".to_string(),
            };
    
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(
                    auth_handler.config.jwt.secret.as_bytes(),
                ),
            )
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
            let response = super::ApiResponse::success(token);
            Ok(HttpResponse::Ok().json(response))
        } else {
            let response = super::ApiResponse::<String>::error("Invalid credentials");
            Ok(HttpResponse::Unauthorized().json(response))
        }
    }
    


    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt.secret.as_bytes()),
            &Validation::default()
        ).map(|token_data| token_data.claims)
    }

    pub async fn check_auth(&self, req: HttpRequest) -> Result<Claims, actix_web::Error> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .ok_or_else(|| actix_web::error::ErrorUnauthorized("No authorization header"))?;

        let auth_str = auth_header
            .to_str()
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid authorization header"))?;

        if !auth_str.starts_with("Bearer ") {
            return Err(actix_web::error::ErrorUnauthorized("Invalid token format"));
        }

        let token = &auth_str[7..];
        self.verify_token(token)
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))
    }

    pub async fn refresh_token(&self, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
        let claims = self.check_auth(req).await?;

        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(
                self.config.jwt.expiration as i64
            ))
            .expect("Error calculando expiración")
            .timestamp();

        let new_claims = Claims {
            sub: claims.sub,
            exp: expiration,
            role: claims.role,
        };

        let new_token = encode(
            &Header::default(),
            &new_claims,
            &EncodingKey::from_secret(
                self.config.jwt.secret.as_bytes()
            ),
        ).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        let response = super::ApiResponse::success(new_token);
        Ok(HttpResponse::Ok().json(response))
    }
        

    // Añadir método de logout
    pub async fn logout(
        auth_handler: web::Data<AuthHandler>,
        req: HttpRequest,
    ) -> Result<HttpResponse, actix_web::Error> {
        // Implementar lógica de logout
        Ok(HttpResponse::Ok().json(super::ApiResponse::<()>::success(())))
    }
}
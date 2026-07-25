//! JWT auth middleware for app gateway routes.

use jsonwebtoken::{decode, DecodingKey, Validation};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wallet_error::AppError;

use crate::GwState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    #[serde(default)]
    pub iss: String,
}

pub fn verify_jwt(token: &str, secret: &str, issuer: &str) -> Result<Claims, AppError> {
    let mut validation = Validation::default();
    validation.set_issuer(&[issuer]);
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|d| d.claims)
    .map_err(|_| AppError::Unauthorized)
}

/// Require a valid JWT on protected routes.
#[handler]
pub async fn require_app_jwt(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    flow: &mut FlowCtrl,
) {
    let auth = req
        .headers()
        .get(salvo::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = match auth.and_then(|a| a.strip_prefix("Bearer ")) {
        Some(t) => t,
        None => {
            res.render(AppError::Unauthorized);
            flow.skip_rest();
            return;
        }
    };

    let st = depot
        .get_typed::<Arc<GwState>>()
        .expect("GwState not inserted");

    if verify_jwt(token, &st.cfg.jwt.secret, &st.cfg.jwt.issuer).is_err() {
        res.render(AppError::Unauthorized);
        flow.skip_rest();
        return;
    }

    flow.call_next(req, depot, res).await;
}

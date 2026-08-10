use jsonwebtoken::{decode, DecodingKey, Validation};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wallet_error::AppError;

use crate::AdminState;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    #[serde(default)]
    roles: Vec<String>,
}

/// Lightweight RBAC gate (Casbin-compatible role claim check).
#[handler]
pub async fn require_admin_jwt(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    flow: &mut FlowCtrl,
) {
    if req.uri().path() == "/healthz" {
        flow.call_next(req, depot, res).await;
        return;
    }

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

    let st = match depot.get_typed::<Arc<AdminState>>() {
        Ok(s) => s.clone(),
        Err(_) => {
            res.render(AppError::internal("state not initialized"));
            flow.skip_rest();
            return;
        }
    };

    let secret = match st.cfg.jwt.secret() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(error = %e, "jwt secret not configured");
            res.render(AppError::internal("server misconfigured"));
            flow.skip_rest();
            return;
        }
    };

    let mut validation = Validation::default();
    validation.set_issuer(&[st.cfg.jwt.issuer.as_str()]);
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation)
        .map_err(|_| AppError::Unauthorized);

    match data {
        Ok(d) => {
            if !d.claims.roles.iter().any(|r| r == "admin" || r == "*") {
                res.render(AppError::Forbidden);
                flow.skip_rest();
                return;
            }
        }
        Err(e) => {
            res.render(e);
            flow.skip_rest();
            return;
        }
    }

    flow.call_next(req, depot, res).await;
}

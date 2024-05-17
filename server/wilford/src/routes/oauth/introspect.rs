//! Introspect endpoint in accordance with RFC7662

use crate::routes::appdata::WDatabase;
use crate::routes::auth::ConstantAccessTokenAuth;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use database::oauth2_client::AccessToken;
use database::user::User;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Deserialize)]
pub struct Form {
    token: String,
    scope: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    active: bool,
    scope: String,
    client_id: String,
    username: String,
    token_type: String,
    exp: i64,
    iat: i64,
    nbf: i64,
    /// Espo user id
    sub: String,
}

#[derive(Debug, Error)]
pub enum IntrospectError {
    #[error("invalid_token")]
    InvalidToken,
    #[error("insufficient_scope")]
    InsufficientScope,
    #[error("{0}")]
    Database(#[from] database::driver::Error),
    #[error("Internal error")]
    Internal,
}

impl ResponseError for IntrospectError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::InsufficientScope => StatusCode::FORBIDDEN,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let error = match self {
            Self::InvalidToken => "invalid_token",
            Self::InsufficientScope => "insufficient_scope",
            Self::Database(_) => "server_error",
            Self::Internal => "server_error",
        };

        #[derive(Serialize)]
        struct Response<'a> {
            error: &'a str,
        }

        HttpResponse::build(self.status_code()).json(&Response { error })
    }
}

pub async fn introspect(
    database: WDatabase,
    _: ConstantAccessTokenAuth,
    form: web::Form<Form>,
) -> Result<web::Json<Response>, IntrospectError> {
    let token = AccessToken::get_by_token(&database, &form.token)
        .await?
        .ok_or(IntrospectError::InvalidToken)?;

    if let Some(scope) = &form.scope {
        let requested = HashSet::from_iter(scope.split(" ").map(|c| c.to_string()));

        let have = token.scopes();
        if !have.is_superset(&requested) {
            return Err(IntrospectError::InsufficientScope);
        }
    }

    let user = User::get_by_id(&database, &token.user_id)
        .await?
        .ok_or(IntrospectError::Internal)?;

    Ok(web::Json(Response {
        active: true,
        scope: token.scopes.unwrap_or_default(),
        client_id: token.client_id,
        username: user.name,
        token_type: "bearer".to_string(),
        exp: token.expires_at,
        iat: token.issued_at,
        nbf: token.issued_at,
        sub: token.user_id,
    }))
}

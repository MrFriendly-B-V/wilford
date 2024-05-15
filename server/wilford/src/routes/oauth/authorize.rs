use crate::response_types::{Redirect, Uncached};
use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::oauth::{OAuth2AuthorizationResponse, OAuth2Error, OAuth2ErrorKind};
use actix_web::web;
use database::oauth2_client::{AuthorizationType, OAuth2Client};
use serde::Deserialize;
use tracing::warn;

#[derive(Deserialize)]
pub struct Query {
    response_type: ResponseType,
    client_id: String,
    redirect_uri: String,
    scope: Option<String>,
    state: Option<String>,
    nonce: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum ResponseType {
    #[serde(rename(deserialize = "code"))]
    /// Authorization Code flow
    /// [RFC6749 Section 4.1](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1)
    Code,
    #[serde(rename(deserialize = "token"))]
    /// Implicit Flow
    /// [RFC6749 Section 4.2](https://datatracker.ietf.org/doc/html/rfc6749#section-4.2)
    Token,
    #[serde(rename(deserialize = "id_token token"))]
    IdToken,
}

pub async fn authorize(
    database: WDatabase,
    config: WConfig,
    query: web::Query<Query>,
) -> OAuth2AuthorizationResponse<Uncached<Redirect>> {
    // Required for the `id_token` response type
    // OpenID Connect Core specification, section 3.2.2.1
    if query.response_type.eq(&ResponseType::IdToken) && query.nonce.is_none() {
        return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
            OAuth2ErrorKind::InvalidRequest,
            &query.redirect_uri,
            query.state.as_deref(),
        ));
    }

    // Get the OAuth2 client
    let client = match OAuth2Client::get_by_client_id(&database, &query.client_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
                OAuth2ErrorKind::UnauthorizedClient,
                &query.redirect_uri,
                query.state.as_deref(),
            ))
        }
        Err(e) => {
            warn!("{e}");
            return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
                OAuth2ErrorKind::ServerError,
                &query.redirect_uri,
                query.state.as_deref(),
            ));
        }
    };

    // Check redirect URI
    if client.redirect_uri.ne(&query.redirect_uri) {
        return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
            OAuth2ErrorKind::UnauthorizedClient,
            &query.redirect_uri,
            query.state.as_deref(),
        ));
    }

    let pending_authorization = client
        .new_pending_authorization(
            &database,
            query.scope.clone(),
            query.state.clone(),
            rt_to_at(&query.response_type),
            query.nonce.clone(),
        )
        .await;

    let pending_authorization = match pending_authorization {
        Ok(pa) => pa,
        Err(e) => {
            warn!("{e}");
            return OAuth2AuthorizationResponse::Err(OAuth2Error::new(
                OAuth2ErrorKind::ServerError,
                &query.redirect_uri,
                query.state.as_deref(),
            ));
        }
    };

    // Redirect to login page
    OAuth2AuthorizationResponse::Ok(Uncached::new(Redirect::new(format!(
        "{}?authorization={}",
        config.http.ui_login_path,
        pending_authorization.id(),
    ))))
}

fn rt_to_at(rt: &ResponseType) -> AuthorizationType {
    match rt {
        ResponseType::Code => AuthorizationType::AuthorizationCode,
        ResponseType::Token => AuthorizationType::Implicit,
        ResponseType::IdToken => AuthorizationType::IdToken,
    }
}

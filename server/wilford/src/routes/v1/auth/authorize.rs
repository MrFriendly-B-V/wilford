use actix_web::cookie::time::OffsetDateTime;
use actix_web::web;
use database::driver::Database;
use serde::{Deserialize, Serialize};
use tap::TapFallible;
use tracing::{instrument, warn};

use database::oauth2_client::{
    create_id_token, AccessToken, AuthorizationType, JwtSigningAlgorithm,
    OAuth2AuthorizationCodeCreationError, OAuth2Client, OAuth2PendingAuthorization,
};
use database::user::User;

use crate::response_types::{MaybeCookie, Redirect, SetCookie};
use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::oauth::{OAuth2AuthorizationResponse, OAuth2Error, OAuth2ErrorKind};
use crate::routes::WOidcSigningKey;

#[derive(Deserialize)]
pub struct Query {
    authorization: String,
    grant: bool,
}

#[instrument(skip_all)]
pub async fn authorize(
    database: WDatabase,
    oidc_signing_key: WOidcSigningKey,
    config: WConfig,
    query: web::Query<Query>,
) -> WebResult<MaybeCookie<'static, OAuth2AuthorizationResponse<Redirect>>> {
    let pending_authorization =
        OAuth2PendingAuthorization::get_by_id(&database, &query.authorization)
            .await?
            .ok_or(WebErrorKind::NotFound)?;

    let client = OAuth2Client::get_by_client_id(&database, &pending_authorization.client_id())
        .await?
        .ok_or(WebErrorKind::NotFound)?;

    if !query.grant {
        return Ok(MaybeCookie::none(OAuth2AuthorizationResponse::Err(
            OAuth2Error::new(
                OAuth2ErrorKind::AccessDenied,
                &client.redirect_uri,
                pending_authorization.state().as_deref(),
            ),
        )));
    }

    let state = pending_authorization.state().clone();
    let res = match pending_authorization.ty() {
        AuthorizationType::AuthorizationCode => {
            let authorization = client
                .new_authorization_code(&database, pending_authorization)
                .await
                .map_err(|e| match e {
                    OAuth2AuthorizationCodeCreationError::Sqlx(e) => WebErrorKind::Database(e),
                    OAuth2AuthorizationCodeCreationError::Unauthorized => {
                        WebErrorKind::InvalidInternalState
                    }
                })?;

            #[derive(Serialize)]
            struct RedirectQuery {
                code: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                state: Option<String>,
            }

            let url = format!(
                "{}?{}",
                client.redirect_uri,
                serde_qs::to_string(&RedirectQuery {
                    code: authorization.code,
                    state,
                })
                .expect("Serializing query string"),
            );

            MaybeCookie::none(OAuth2AuthorizationResponse::Ok(Redirect::new(url)))
        }
        AuthorizationType::Implicit => {
            let access_token = new_access_token(&client, pending_authorization, &database).await?;

            let url = format!(
                "{}#{}",
                client.redirect_uri,
                create_implicit_fragment(None, access_token.clone(), state),
            );

            MaybeCookie::some(SetCookie::new(
                "Authorization",
                format!("Bearer {}", access_token.token),
                OAuth2AuthorizationResponse::Ok(Redirect::new(url)),
            ))
        }
        AuthorizationType::IdToken => {
            let nonce = pending_authorization.nonce().clone();
            let access_token = new_access_token(&client, pending_authorization, &database).await?;

            let url = format!(
                "{}#{}",
                client.redirect_uri,
                create_implicit_fragment(
                    Some(
                        create_id_token(
                            config.oidc_issuer.clone(),
                            &client,
                            &User::get_by_id(&database, &access_token.user_id)
                                .await?
                                .ok_or(WebErrorKind::InternalServerError)?,
                            &oidc_signing_key.0,
                            &access_token,
                            nonce,
                            JwtSigningAlgorithm::RS256,
                        )
                        .tap_err(|e| warn!("Failed to create ID token: {e}"))
                        .map_err(|_| WebErrorKind::InternalServerError)?
                    ),
                    access_token.clone(),
                    state
                )
            );

            MaybeCookie::some(SetCookie::new(
                "Authorization",
                format!("Bearer {}", access_token.token),
                OAuth2AuthorizationResponse::Ok(Redirect::new(url)),
            ))
        }
    };

    Ok(res)
}

/// Create a new OAuth2 access token.
/// Useful for the OAuth2 Implicit flow and the OpenID Connect IdToken flow.
async fn new_access_token(
    client: &OAuth2Client,
    pending_authorization: OAuth2PendingAuthorization,
    database: &Database,
) -> WebResult<AccessToken> {
    Ok(client
        .new_access_token(&database, pending_authorization)
        .await
        .map_err(|e| match e {
            OAuth2AuthorizationCodeCreationError::Sqlx(e) => WebErrorKind::Database(e),
            OAuth2AuthorizationCodeCreationError::Unauthorized => {
                WebErrorKind::InvalidInternalState
            }
        })?)
}

#[derive(Serialize)]
struct RedirectFragment {
    access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    token_type: &'static str,
    expires_in: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_token: Option<String>,
}

/// Create the fragment string used for the OAuth2 implicit flow and the OpenID Connect IdToken flow.
/// The `id_token` string should only be supplied for the OpenID Connect IdToken flow.
fn create_implicit_fragment(
    id_token: Option<String>,
    access_token: AccessToken,
    state: Option<String>,
) -> String {
    serde_qs::to_string(&RedirectFragment {
        access_token: access_token.token,
        token_type: "bearer",
        expires_in: access_token.expires_at - OffsetDateTime::now_utc().unix_timestamp(),
        state,
        id_token,
    })
    .expect("Serializing query string")
}

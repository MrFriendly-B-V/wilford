use crate::routes::appdata::WDatabase;
use crate::routes::oauth::OAuth2ErrorKind;
use actix_web::cookie::time::OffsetDateTime;
use actix_web::web;
use database::oauth2_client::{OAuth2AuthorizationCode, OAuth2Client, RefreshToken};
use serde::{Deserialize, Serialize};
use tap::TapFallible;
use tracing::warn;
use crate::response_types::Uncached;

#[derive(Deserialize)]
pub struct Form {
    grant_type: GrantType,
    code: Option<String>,
    redirect_uri: String,
    client_id: String,
    refresh_token: Option<String>,
    client_secret: String,
}

#[derive(Deserialize)]
pub enum GrantType {
    #[serde(rename(deserialize = "authorization_code"))]
    AuthorizationCode,
    #[serde(rename(deserialize = "refresh_token"))]
    RefreshToken,
}

#[derive(Serialize)]
pub struct Response {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: String,
    scope: String,
}

pub async fn token(
    database: WDatabase,
    form: web::Form<Form>,
) -> Result<Uncached<web::Json<Response>>, OAuth2ErrorKind> {
    let client = OAuth2Client::get_by_client_id(&database, &form.client_id)
        .await
        .tap_err(|e| warn!("{e}"))
        .map_err(|_| OAuth2ErrorKind::ServerError)?
        .ok_or(OAuth2ErrorKind::UnauthorizedClient)?;

    if client.client_secret.ne(&form.client_secret) {
        return Err(OAuth2ErrorKind::UnauthorizedClient);
    }

    if client.redirect_uri.ne(&form.redirect_uri) {
        return Err(OAuth2ErrorKind::UnauthorizedClient);
    }

    match form.grant_type {
        GrantType::AuthorizationCode => {
            let code = match &form.code {
                Some(c) => c,
                None => return Err(OAuth2ErrorKind::InvalidRequest),
            };

            let authorization = OAuth2AuthorizationCode::get_by_code(&database, code)
                .await
                .tap_err(|e| warn!("{e}"))
                .map_err(|_| OAuth2ErrorKind::ServerError)?
                .ok_or(OAuth2ErrorKind::InvalidGrant)?;

            if authorization.client_id.ne(&client.client_id) {
                return Err(OAuth2ErrorKind::InvalidGrant);
            }

            if OffsetDateTime::now_utc().unix_timestamp() > authorization.expires_at {
                return Err(OAuth2ErrorKind::InvalidGrant);
            }

            let (atoken, rtoken) = client
                .new_token_pair(&database, authorization)
                .await
                .tap_err(|e| warn!("{e}"))
                .map_err(|_| OAuth2ErrorKind::ServerError)?;

            Ok(Uncached::new(web::Json(Response {
                access_token: atoken.token,
                token_type: "bearer".to_string(),
                scope: atoken.scopes.unwrap_or_default(),
                expires_in: OffsetDateTime::now_utc().unix_timestamp() - atoken.expires_at,
                refresh_token: rtoken.token,
            })))
        }
        GrantType::RefreshToken => {
            let rtoken = match &form.refresh_token {
                Some(r) => r,
                None => return Err(OAuth2ErrorKind::InvalidRequest),
            };

            let rtoken = RefreshToken::get_by_token(&database, &rtoken)
                .await
                .tap_err(|e| warn!("{e}"))
                .map_err(|_| OAuth2ErrorKind::ServerError)?
                .ok_or(OAuth2ErrorKind::InvalidGrant)?;

            if client.client_id.ne(&rtoken.client_id) {
                return Err(OAuth2ErrorKind::InvalidGrant);
            }

            let atoken = client
                .refresh_access_token(&database, &rtoken)
                .await
                .tap_err(|e| warn!("{e}"))
                .map_err(|_| OAuth2ErrorKind::ServerError)?;

            Ok(Uncached::new(web::Json(Response {
                access_token: atoken.token,
                token_type: "bearer".to_string(),
                expires_in: atoken.expires_at - OffsetDateTime::now_utc().unix_timestamp(),
                scope: atoken.scopes.unwrap_or_default(),
                refresh_token: rtoken.token,
            })))
        }
    }
}

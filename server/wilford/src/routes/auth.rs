use crate::espo::user::EspoUser;
use crate::routes::appdata::{WDatabase, WEspo};
use crate::routes::error::{WebError, WebResult};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use database::constant_access_tokens::ConstantAccessToken;
use database::oauth2_client::{AccessToken, OAuth2Client};
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use tap::TapOptional;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct Auth {
    pub espo_user_id: String,
    pub name: String,
    pub is_admin: bool,
    token: AccessToken,
}

impl FromRequest for Auth {
    type Error = WebError;
    type Future = Pin<Box<dyn Future<Output = WebResult<Self>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        let database = req
            .app_data::<WDatabase>()
            .expect("Getting AppData for type WDatabase")
            .clone();
        let espo_client = req
            .app_data::<WEspo>()
            .expect("Getting AppData for type WEspo")
            .clone();

        Box::pin(async move {
            let token = get_authorization_token(&req)?;
            let client = OAuth2Client::list(&database)
                .await?
                .into_iter()
                .find(|f| f.is_internal)
                .tap_none(|| warn!("No OAuth2 client exists matching `is_internal == true`"))
                .ok_or(WebError::InvalidInternalState)?;

            let token_info =
                match AccessToken::get_with_validation(&database, &token, &client).await? {
                    Some(v) => v,
                    None => return Err(WebError::Unauthorized),
                };

            let espo_user = EspoUser::get_by_id(&espo_client, &token_info.espo_user_id)
                .await
                .map_err(|e| WebError::Espo(e))?;

            Ok(Self {
                espo_user_id: espo_user.id,
                name: espo_user.name,
                is_admin: espo_user.user_type.eq("admin"),
                token: token_info,
            })
        })
    }
}

impl Auth {
    #[must_use]
    pub fn has_scope(&self, scope: &str) -> bool {
        self.token.scopes().contains(scope)
    }

    pub fn scopes(&self) -> HashSet<String> {
        self.token.scopes()
    }
}

/// Authentication using a constant token.
/// These tokens are created manually.
pub struct ConstantAccessTokenAuth {
    pub name: String,
    pub token: String,
}

impl FromRequest for ConstantAccessTokenAuth {
    type Error = WebError;
    type Future = Pin<Box<dyn Future<Output = WebResult<Self>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        let database = req
            .app_data::<WDatabase>()
            .expect("Getting AppData for type WDatabase")
            .clone();

        Box::pin(async move {
            let token = get_authorization_token(&req)?;
            let cat = ConstantAccessToken::get_by_token(&database, &token)
                .await?
                .ok_or(WebError::Unauthorized)?;

            Ok(Self {
                name: cat.name,
                token: cat.token,
            })
        })
    }
}

fn get_authorization_token(req: &HttpRequest) -> WebResult<String> {
    let header = req
        .headers()
        .get("Authorization")
        .map(|v| v.to_str().map(|v| v.to_string()));

    match header {
        Some(Ok(h)) if h.starts_with("Bearer ") => return Ok(h.replace("Bearer ", "")),
        _ => {}
    }

    let cookie = req.cookie("Authorization").map(|c| c.value().to_string());

    match cookie {
        Some(c) if c.starts_with("Bearer ") => return Ok(c.replace("Bearer ", "")),
        _ => {}
    }

    Err(WebError::Unauthorized)
}

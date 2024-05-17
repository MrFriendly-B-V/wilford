use crate::espo::user::{EspoUser, LoginStatus};
use crate::routes::appdata::{WConfig, WDatabase, WEspo};
use crate::routes::error::{WebError, WebResult};
use actix_web::web;
use database::oauth2_client::OAuth2PendingAuthorization;
use database::user::User;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct Request {
    authorization: String,
    username: String,
    password: String,
    totp_code: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    status: bool,
    totp_required: bool,
}

pub async fn login(
    database: WDatabase,
    config: WConfig,
    espo: WEspo,
    payload: web::Json<Request>,
) -> WebResult<web::Json<Response>> {
    let authorization = OAuth2PendingAuthorization::get_by_id(&database, &payload.authorization)
        .await?
        .ok_or(WebError::NotFound)?;

    let login = EspoUser::try_login(
        &config.espo.host,
        &payload.username,
        &payload.password,
        payload.totp_code.as_deref(),
    )
    .await
    .map_err(|e| WebError::Espo(e))?;

    // OAuth2 defines `scope` to be all scopes, seperated by a ' ' (space char)
    // Where duplicates can be ignored.
    let scope_set = authorization
        .scopes()
        .clone()
        .map(|s| s.split(" ").map(|c| c.to_string()).collect::<HashSet<_>>())
        .unwrap_or_default();

    match login {
        LoginStatus::Ok(id) => {
            // Create a user if it doesn't exist
            // If it does, check if all scopes are allowed.
            // Only exceptions to this are Espo admins, they may have all scopes,
            // and the OIDC scopes
            match User::get_by_id(&database, &id).await? {
                Some(user) if user.is_admin => {}
                Some(user) => {
                    let permitted_scopes =
                        HashSet::from_iter(user.list_permitted_scopes(&database).await?);

                    let oidc_scopes = oidc_scopes();
                    let allowed_scopes = permitted_scopes
                        .union(&oidc_scopes)
                        .map(|c| c.to_string())
                        .collect::<HashSet<_>>();

                    let disallowed_scopes = scope_set
                        .difference(&allowed_scopes)
                        .collect::<HashSet<_>>();

                    if !disallowed_scopes.is_empty() {
                        return Err(WebError::Forbidden);
                    }
                }
                None => {
                    let espo_user = EspoUser::get_by_id(&espo, &id)
                        .await
                        .map_err(|e| WebError::Espo(e))?;

                    let user = User::new(
                        &database,
                        id.clone(),
                        espo_user.name,
                        espo_user.email_address,
                        espo_user.user_type.eq("admin"),
                    )
                    .await?;

                    // No permitted scopes are granted yet
                    if !user.is_admin {
                        // Remove the OIDC scopes
                        let oidc_scopes = oidc_scopes();
                        let disallowed_scopes =
                            scope_set.difference(&oidc_scopes).collect::<HashSet<_>>();

                        if !disallowed_scopes.is_empty() {
                            return Err(WebError::Forbidden);
                        }
                    }
                }
            }

            authorization
                .set_user_id(&database, &id)
                .await
                .map_err(|_| WebError::BadRequest)?;

            Ok(web::Json(Response {
                status: true,
                totp_required: false,
            }))
        }
        LoginStatus::SecondStepRequired => Ok(web::Json(Response {
            status: false,
            totp_required: true,
        })),
        LoginStatus::Err => Ok(web::Json(Response {
            status: false,
            totp_required: false,
        })),
    }
}

fn oidc_scopes() -> HashSet<String> {
    HashSet::from_iter([
        "openid".to_string(),
        "profile".to_string(),
        "email".to_string(),
    ])
}

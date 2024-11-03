use crate::authorization_backends::{AuthorizationBackend, CheckResult};
use crate::routes::appdata::WDatabase;
use crate::routes::error::{WebError, WebErrorKind, WebResult};
use crate::routes::WAuthorizationProvider;
use actix_web::web;
use database::driver::Database;
use database::oauth2_client::OAuth2PendingAuthorization;
use database::user::User;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct Request {
    /// The identifier for the pending authorization
    authorization: String,
    /// The login username
    username: String,
    /// The login password
    password: String,
    /// If required, the TOTP code
    totp_code: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    /// The result of the login request. `true` indicates the login was successful.
    /// `false` indicates the credentials provided are invalid, or further action is needed.
    status: bool,
    /// Whether 2FA is required.
    totp_required: bool,
}

#[instrument(skip_all)]
pub async fn login(
    database: WDatabase,
    payload: web::Json<Request>,
    authorization_provider: WAuthorizationProvider,
) -> WebResult<web::Json<Response>> {
    let authorization = OAuth2PendingAuthorization::get_by_id(&database, &payload.authorization)
        .await?
        .ok_or(WebErrorKind::NotFound)?;

    // Validate the provided login credentials
    let login_check = authorization_provider
        .check_credentials(
            &payload.username,
            &payload.password,
            payload.totp_code.as_deref(),
        )
        .await
        .map_err(|e| WebError::new(WebErrorKind::AuthorizationProvider(e)))?;

    match login_check {
        CheckResult::Ok(user) => {
            // Create a user if it doesn't exist
            // If it does, check if all scopes are allowed.
            // Only exceptions to this:
            // - admins, they may have all scopes,
            // - oidc scopes, they are always allowed.
            match User::get_by_id(&database, &user.id).await? {
                Some(user) if user.is_admin => {}
                Some(user) => {
                    // We check that the scopes requested in this authorization flow are actually
                    // allowed to be granted to the requesting user by the administrator.

                    let requested_scopes = requested_scope_set(&authorization);
                    let permitted_scopes = user_permitted_scopes(&user, &database).await?;

                    if !scope_request_permitted(&permitted_scopes, &requested_scopes) {
                        return Err(WebErrorKind::Forbidden.into());
                    }
                }
                None => {
                    let user = User::new(
                        &database,
                        user.id.clone(),
                        user.name,
                        user.email,
                        user.is_admin,
                    )
                    .await?;

                    // No permitted scopes are granted yet
                    if !user.is_admin {
                        let requested_scopes = requested_scope_set(&authorization);
                        let permitted_scopes = user_permitted_scopes(&user, &database).await?;

                        if !scope_request_permitted(&permitted_scopes, &requested_scopes) {
                            return Err(WebErrorKind::Forbidden.into());
                        }
                    }
                }
            }

            authorization
                .set_user_id(&database, &user.id)
                .await
                .map_err(|_| WebErrorKind::BadRequest)?;

            Ok(web::Json(Response {
                status: true,
                totp_required: false,
            }))
        }
        CheckResult::TwoFactorRequired => Ok(web::Json(Response {
            status: false,
            totp_required: true,
        })),
        CheckResult::Invalid => Ok(web::Json(Response {
            status: false,
            totp_required: false,
        })),
    }
}

/// Check if the requested scopes are contained in the set of permitted scopes. Returns `true`
/// if this is the case.
fn scope_request_permitted(permitted: &HashSet<String>, requested: &HashSet<String>) -> bool {
    let disallowed_scopes = requested.difference(permitted).collect::<HashSet<_>>();

    disallowed_scopes.is_empty()
}

/// Get the scopes the user has been granted by the administrator.
/// This set also contains the OIDC scopes, which are always allowed.
///
/// # Errors
///
/// If a database error occurs while fetching the list of permitted scopes.
async fn user_permitted_scopes(
    user: &User,
    database: &Database,
) -> Result<HashSet<String>, database::driver::Error> {
    let permitted_scopes = HashSet::from_iter(user.list_permitted_scopes(database).await?);

    let oidc_scopes = oidc_scopes();
    let allowed_scopes = permitted_scopes
        .union(&oidc_scopes)
        .map(|c| c.to_string())
        .collect::<HashSet<_>>();

    Ok(allowed_scopes)
}

/// Get the set of scopes rqeuested in the authorization request
fn requested_scope_set(authorization: &OAuth2PendingAuthorization) -> HashSet<String> {
    authorization
        .scopes()
        .clone()
        .map(|s| s.split(" ").map(|c| c.to_string()).collect::<HashSet<_>>())
        .unwrap_or_default()
}

/// The set of scopes which are always allowed.
/// These are essential to the OIDC flow and should not be used for access control.
fn oidc_scopes() -> HashSet<String> {
    HashSet::from_iter([
        "openid".to_string(),
        "profile".to_string(),
        "email".to_string(),
    ])
}

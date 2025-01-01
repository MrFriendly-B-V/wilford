use crate::authorization::combined::{
    CombinedAuthorizationProvider, CombinedAuthorizationProviderError,
};
use crate::authorization::espo::EspoAuthorizationProviderError;
use crate::authorization::local_provider::LocalAuthorizationProviderError;
use crate::authorization::{AuthorizationError, AuthorizationProvider, UserInformation};
use crate::routes::appdata::{WConfig, WDatabase};
use crate::routes::error::{WebErrorKind, WebResult};
use actix_web::web;
use database::driver::Database;
use database::oauth2_client::OAuth2PendingAuthorization;
use database::user::User;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tap::TapFallible;
use tracing::{instrument, warn, warn_span, Instrument};

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

#[instrument(skip_all)]
pub async fn login(
    database: WDatabase,
    config: WConfig,
    payload: web::Json<Request>,
) -> WebResult<web::Json<Response>> {
    // Get the authorization assocated with the provided token
    let authorization = OAuth2PendingAuthorization::get_by_id(&database, &payload.authorization)
        .instrument(warn_span!("OAuth2PendingAuthorization::get_by_id"))
        .await
        .tap_err(|e| warn!("{e}"))?
        .ok_or(WebErrorKind::NotFound)?;

    // Get the provider backend
    let auth_provider = CombinedAuthorizationProvider::new(&config, &database);

    // Check the credentials and handle results
    let validation_result = match auth_provider
        .validate_credentials(
            &payload.username,
            &payload.password,
            payload.totp_code.as_deref(),
        )
        .instrument(warn_span!("auth_provider::validate_credentials"))
        .await
        .tap_err(|e| warn!("{e}"))
    {
        Err(AuthorizationError::InvalidCredentials) => {
            return Ok(web::Json(Response {
                status: false,
                totp_required: false,
            }))
        }
        Err(AuthorizationError::TotpNeeded) => {
            return Ok(web::Json(Response {
                status: false,
                totp_required: true,
            }))
        }
        // Should not happen here, but handle anyway.
        Err(AuthorizationError::UnsupportedOperation) => {
            return Err(WebErrorKind::InternalServerError.into())
        }
        // Should not happen here, but handle anyway.
        Err(AuthorizationError::AlreadyExists) => {
            return Err(WebErrorKind::InternalServerError.into())
        }
        Err(AuthorizationError::Other(e)) => {
            warn!("Authorization validation failed: {e}");

            // Could we do this better? Ugly.
            return Err(match e {
                CombinedAuthorizationProviderError::EspoCrm(e) => match e {
                    EspoAuthorizationProviderError::Database(e) => e.into(),
                    EspoAuthorizationProviderError::Espocrm(e) => WebErrorKind::Espo(e).into(),
                },
                CombinedAuthorizationProviderError::Local(e) => match e {
                    LocalAuthorizationProviderError::Database(e) => e.into(),
                    LocalAuthorizationProviderError::Hashing(_) => {
                        WebErrorKind::InternalServerError.into()
                    }
                },
            });
        }
        Ok(user_information) => user_information,
    };

    // Check if any scopes were requested that the user should not be allowed to access
    // This check is skipped for admins.
    // For optimizations, we evaluate the is_admin check first, followed by the scope check. Due to
    // short-circuiting behaviour, the scope check is only evaluated if the user is _not_ an admin.
    // We use the lambda function to reduce the complecity of the if statement.
    let scope_check = || {
        are_scopes_allowed(
            &database,
            &authorization,
            &validation_result.user_information,
        )
        .instrument(warn_span!("scope_check"))
    };

    if !validation_result.user_information.is_admin && !scope_check().await? {
        return Err(WebErrorKind::Forbidden.into());
    }

    // Mark the authorization as authorized.
    authorization
        .set_user_id(&database, &validation_result.user_information.id)
        .instrument(warn_span!("authorization::set_user_id"))
        .await
        .tap_err(|e| warn!("{e}"))
        .map_err(|_| WebErrorKind::BadRequest)?;

    Ok(web::Json(Response {
        status: true,
        totp_required: false,
    }))
}

#[instrument(skip_all)]
async fn are_scopes_allowed(
    database: &Database,
    authorization: &OAuth2PendingAuthorization,
    user_information: &UserInformation,
) -> WebResult<bool> {
    // Get the user in the database
    let user = User::get_by_id(&database, &user_information.id)
        .await?
        .ok_or(WebErrorKind::InternalServerError)?;

    // Check which scopes are granted to the user
    let permitted_scopes = HashSet::from_iter(user.list_permitted_scopes(&database).await?);

    // The set of allowed scopes are the scopes granted to the user by an admin
    // and the oidc scopes, which are always allowed
    let oidc_scopes = oidc_scopes();
    let allowed_scopes = permitted_scopes
        .union(&oidc_scopes)
        .map(|c| c.to_string())
        .collect::<HashSet<_>>();

    // Parse the requested scopes to a set.
    // OAuth2 defines `scope` to be all scopes, seperated by a ' ' (space char)
    // Where duplicates can be ignored.
    let requested_scopes = authorization
        .scopes()
        .clone()
        .map(|s| s.split(" ").map(|c| c.to_string()).collect::<HashSet<_>>())
        .unwrap_or_default();

    // The difference between the requested and allowed is the set of
    // scopes requested, but which are not allowed for the user.
    let disallowed_scopes = requested_scopes
        .difference(&allowed_scopes)
        .collect::<HashSet<_>>();

    // Thus, if the set of dissalowed scopes is empty,
    // the user only requested scopes which they can access,
    // and thus the scope check succeeded.
    Ok(disallowed_scopes.is_empty())
}

fn oidc_scopes() -> HashSet<String> {
    HashSet::from_iter([
        "openid".to_string(),
        "profile".to_string(),
        "email".to_string(),
    ])
}

use crate::routes::auth::Auth;
use crate::routes::error::WebResult;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    /// The scopes allowed for the authorization, space separated.
    scope: String,
}

/// Get information about the OAuth2 token used for authorizations
///
/// # Errors
///
/// If the operation fails
pub async fn token_info(auth: Auth) -> WebResult<web::Json<Response>> {
    Ok(web::Json(Response {
        scope: auth.scopes().into_iter().collect::<Vec<_>>().join(" "),
    }))
}

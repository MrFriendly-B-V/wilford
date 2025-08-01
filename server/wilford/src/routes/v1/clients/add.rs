use actix_web::web;
use serde::Deserialize;

use database::oauth2_client::OAuth2Client;

use crate::response_types::Empty;
use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Deserialize)]
pub struct Request {
    /// The name of an OAuth client
    name: String,
    /// The redirect URI of the client
    redirect_uri: String,
}

/// Add a new OAuth2 client
///
/// # Errors
///
/// - If the user does not have sufficient scopes
/// - If the name is too long; >64
/// - If the name is already used
/// - If the operation fails
pub async fn add(database: WDatabase, auth: Auth, payload: web::Json<Request>) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    if payload.name.len() > 64 {
        return Err(WebErrorKind::BadRequest.into());
    }

    let exists = OAuth2Client::list(&database)
        .await?
        .into_iter()
        .any(|c| c.name.eq(&payload.name));

    if exists {
        return Err(WebErrorKind::BadRequest.into());
    }

    OAuth2Client::new(
        &database,
        payload.name.clone(),
        payload.redirect_uri.clone(),
        false,
    )
    .await?;

    Ok(Empty)
}

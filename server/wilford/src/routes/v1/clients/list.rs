use actix_web::web;
use serde::Serialize;

use database::oauth2_client::OAuth2Client;

use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Serialize)]
pub struct Response {
    /// All clients
    clients: Vec<Client>,
}

#[derive(Serialize)]
pub struct Client {
    /// The name of the client
    name: String,
    /// The redirect URI of the client
    redirect_uri: String,
    /// The OAuth2 `client_id`
    client_id: String,
    /// The OAuth2 `client_secret`
    client_secret: String,
}

/// List all configured OAuth2 clients
///
/// # Errors
///
/// - If the user has insufficient permissions
/// - If the operation fails
pub async fn list(database: WDatabase, auth: Auth) -> WebResult<web::Json<Response>> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    let clients = OAuth2Client::list(&database)
        .await?
        .into_iter()
        // Don't show the internal client in this list
        .filter(|f| !f.is_internal)
        .map(|c| Client {
            name: c.name,
            redirect_uri: c.redirect_uri,
            client_id: c.client_id,
            client_secret: c.client_secret,
        })
        .collect();

    Ok(web::Json(Response { clients }))
}

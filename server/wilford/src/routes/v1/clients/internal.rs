use actix_web::web;
use serde::Serialize;

use database::oauth2_client::OAuth2Client;

use crate::routes::appdata::WDatabase;
use crate::routes::error::{WebErrorKind, WebResult};

#[derive(Serialize)]
pub struct Response {
    /// The name of the client
    name: String,
    /// The ID of the client
    client_id: String,
    /// The redirect uri of the client
    redirect_uri: String,
}

/// Get the client information of the internal OAuth2 client
///
/// # Errors
///
/// If the operation fails
pub async fn internal(database: WDatabase) -> WebResult<web::Json<Response>> {
    let client = OAuth2Client::list(&database)
        .await?
        .into_iter()
        .find(|c| c.is_internal)
        .ok_or(WebErrorKind::InvalidInternalState)?;

    Ok(web::Json(Response {
        name: client.name,
        client_id: client.client_id,
        redirect_uri: client.redirect_uri,
    }))
}

use actix_web::web;
use serde::Serialize;

use database::oauth2_client::OAuth2Client;

use crate::routes::appdata::WDatabase;
use crate::routes::error::{WebErrorKind, WebResult};

#[derive(Serialize)]
pub struct Response {
    name: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

pub async fn internal(database: WDatabase) -> WebResult<web::Json<Response>> {
    let client = OAuth2Client::list(&database)
        .await?
        .into_iter()
        .find(|c| c.is_internal)
        .ok_or(WebErrorKind::InvalidInternalState)?;

    Ok(web::Json(Response {
        name: client.name,
        client_id: client.client_id,
        client_secret: client.client_secret,
        redirect_uri: client.redirect_uri,
    }))
}

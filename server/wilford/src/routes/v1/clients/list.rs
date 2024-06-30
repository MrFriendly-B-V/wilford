use actix_web::web;
use serde::Serialize;

use database::oauth2_client::OAuth2Client;

use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Serialize)]
pub struct Response {
    clients: Vec<Client>,
}

#[derive(Serialize)]
pub struct Client {
    name: String,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

pub async fn list(database: WDatabase, auth: Auth) -> WebResult<web::Json<Response>> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    let clients = OAuth2Client::list(&database)
        .await?
        .into_iter()
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

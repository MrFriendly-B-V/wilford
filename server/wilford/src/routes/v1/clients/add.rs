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
    name: String,
    redirect_uri: String,
}

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
        .find(|c| c.name.eq(&payload.name))
        .is_some();

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

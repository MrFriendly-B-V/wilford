use actix_web::web;
use serde::Deserialize;

use database::constant_access_tokens::ConstantAccessToken;

use crate::response_types::Empty;
use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Deserialize)]
pub struct Request {
    name: String,
}

pub async fn add(database: WDatabase, auth: Auth, payload: web::Json<Request>) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    let exists = ConstantAccessToken::list(&database)
        .await?
        .into_iter()
        .find(|f| f.name.eq(&payload.name))
        .is_some();

    if exists {
        return Err(WebErrorKind::BadRequest.into());
    }

    if payload.name.len() > 64 {
        return Err(WebErrorKind::BadRequest.into());
    }

    ConstantAccessToken::new(&database, payload.name.clone()).await?;
    Ok(Empty)
}

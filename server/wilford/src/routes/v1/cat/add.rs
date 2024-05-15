use crate::response_types::Empty;
use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebError, WebResult};
use crate::routes::v1::MANAGE_SCOPE;
use actix_web::web;
use database::constant_access_tokens::ConstantAccessToken;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    name: String,
}

pub async fn add(database: WDatabase, auth: Auth, payload: web::Json<Request>) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebError::Forbidden);
    }

    let exists = ConstantAccessToken::list(&database)
        .await?
        .into_iter()
        .find(|f| f.name.eq(&payload.name))
        .is_some();

    if exists {
        return Err(WebError::BadRequest);
    }

    if payload.name.len() > 64 {
        return Err(WebError::BadRequest);
    }

    ConstantAccessToken::new(&database, payload.name.clone()).await?;
    Ok(Empty)
}

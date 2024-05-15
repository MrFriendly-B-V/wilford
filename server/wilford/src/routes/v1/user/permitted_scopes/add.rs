use crate::response_types::Empty;
use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebError, WebResult};
use crate::routes::v1::MANAGE_SCOPE;
use actix_web::web;
use database::user::User;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Payload {
    /// Espo user ID
    to: String,
    /// Scope to add
    scope: String,
}

pub async fn add(database: WDatabase, auth: Auth, payload: web::Json<Payload>) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebError::Forbidden);
    }

    let user = User::get_by_id(&database, &payload.to)
        .await?
        .ok_or(WebError::NotFound)?;

    let current_scopes = user.list_permitted_scopes(&database).await?;
    if current_scopes.contains(&payload.scope) {
        return Err(WebError::BadRequest);
    }

    user.grant_permitted_scope(&database, &payload.scope)
        .await?;

    Ok(Empty)
}

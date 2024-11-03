use actix_web::web;
use serde::Deserialize;

use database::user::User;

use crate::response_types::Empty;
use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Deserialize)]
pub struct Payload {
    /// Espo user ID
    to: String,
    /// Scope to add
    scope: String,
}

pub async fn add(database: WDatabase, auth: Auth, payload: web::Json<Payload>) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    let user = User::get_by_id(&database, &payload.to)
        .await?
        .ok_or(WebErrorKind::NotFound)?;

    let current_scopes = user.list_permitted_scopes(&database).await?;
    if current_scopes.contains(&payload.scope) {
        return Err(WebErrorKind::BadRequest.into());
    }

    user.grant_permitted_scope(&database, &payload.scope)
        .await?;

    Ok(Empty)
}

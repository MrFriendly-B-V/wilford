use actix_web::web;
use serde::Deserialize;

use database::user::User;

use crate::response_types::Empty;
use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Deserialize)]
pub struct Request {
    /// Espo user ID
    from: String,
    /// The scope to remove
    scope: String,
}

pub async fn remove(
    database: WDatabase,
    auth: Auth,
    payload: web::Json<Request>,
) -> WebResult<Empty> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    let user = User::get_by_id(&database, &payload.from)
        .await?
        .ok_or(WebErrorKind::NotFound)?;
    let current_scops = user.list_permitted_scopes(&database).await?;

    if !current_scops.contains(&payload.scope) {
        return Err(WebErrorKind::NotFound.into());
    }

    user.remove_permitted_scope(&database, &payload.scope)
        .await?;

    Ok(Empty)
}

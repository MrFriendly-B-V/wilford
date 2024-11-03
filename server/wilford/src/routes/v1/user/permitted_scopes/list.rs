use actix_web::web;
use serde::{Deserialize, Serialize};

use database::user::User;

use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Serialize)]
pub struct Response {
    scopes: Vec<String>,
}

#[derive(Deserialize)]
pub struct Query {
    /// Espo user ID
    user: String,
}

pub async fn list(
    database: WDatabase,
    auth: Auth,
    query: web::Query<Query>,
) -> WebResult<web::Json<Response>> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    let user = User::get_by_id(&database, &query.user)
        .await?
        .ok_or(WebErrorKind::NotFound)?;
    let scopes = user.list_permitted_scopes(&database).await?;

    Ok(web::Json(Response { scopes }))
}

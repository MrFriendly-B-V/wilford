use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebError, WebResult};
use crate::routes::v1::MANAGE_SCOPE;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    users: Vec<User>,
}

#[derive(Serialize)]
pub struct User {
    name: String,
    espo_user_id: String,
    is_admin: bool,
}

pub async fn list(database: WDatabase, auth: Auth) -> WebResult<web::Json<Response>> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebError::Forbidden);
    }

    let users = database::user::User::list(&database)
        .await?
        .into_iter()
        .map(|u| User {
            name: u.name,
            espo_user_id: u.user_id,
            is_admin: u.is_admin,
        })
        .collect::<Vec<_>>();

    Ok(web::Json(Response { users }))
}

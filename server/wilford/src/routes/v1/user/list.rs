use actix_web::web;
use serde::Serialize;

use crate::routes::appdata::WDatabase;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::MANAGE_SCOPE;

#[derive(Serialize)]
pub struct Response {
    /// All registered users
    users: Vec<User>,
}

#[derive(Serialize)]
pub struct User {
    /// The name of the user
    name: String,
    /// The user ID
    espo_user_id: String,
    /// Whether the user is an admin
    is_admin: bool,
    /// The email address of the user
    email: String,
}

/// List all users.
///
/// # Errors
///
/// - If the requesting user does not have the required scope.
/// - If the underlying request fails.
pub async fn list(database: WDatabase, auth: Auth) -> WebResult<web::Json<Response>> {
    if !auth.has_scope(MANAGE_SCOPE) {
        return Err(WebErrorKind::Forbidden.into());
    }

    let users = database::user::User::list(&database)
        .await?
        .into_iter()
        .map(|u| User {
            name: u.name,
            email: u.email,
            espo_user_id: u.user_id,
            is_admin: u.is_admin,
        })
        .collect::<Vec<_>>();

    Ok(web::Json(Response { users }))
}

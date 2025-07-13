use crate::routes::auth::Auth;
use crate::routes::error::WebResult;
use crate::routes::WDatabase;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    /// The name of the user
    name: String,
    /// The email address of the user
    email: String,
    /// Whether the user is a super-admin
    is_admin: bool,
    /// The user ID
    espo_user_id: String,
    /// Whether a password change is required
    require_password_change: bool,
}

/// Get information about the user
///
/// # Errors
///
/// If the operation fails
pub async fn info(auth: Auth, database: WDatabase) -> WebResult<web::Json<Response>> {
    Ok(web::Json(Response {
        name: auth.name,
        espo_user_id: auth.user_id,
        is_admin: auth.is_admin,
        require_password_change: auth
            .user
            .password_change_required(&database)
            .await?
            .unwrap_or(false),
        email: auth.user.email,
    }))
}

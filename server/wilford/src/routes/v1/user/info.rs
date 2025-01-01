use crate::routes::auth::Auth;
use crate::routes::error::WebResult;
use crate::routes::WDatabase;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    name: String,
    is_admin: bool,
    espo_user_id: String,
    require_password_change: bool,
}

/// Get information about the user
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
    }))
}

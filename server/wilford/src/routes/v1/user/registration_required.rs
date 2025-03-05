use crate::routes::error::WebResult;
use crate::routes::WDatabase;
use actix_web::web;
use database::user::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    /// Whether the registration of a user is required
    registration_required: bool,
}

/// Whether the registration of a user is required
///
/// # Errors
///
/// If the operation fails
pub async fn registration_required(database: WDatabase) -> WebResult<web::Json<Response>> {
    Ok(web::Json(Response {
        // Require registration of a user if we curently do not have any
        registration_required: User::count(&database).await? == 0,
    }))
}

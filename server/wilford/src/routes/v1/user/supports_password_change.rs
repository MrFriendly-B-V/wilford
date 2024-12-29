use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::routes::auth::Auth;
use crate::routes::{WConfig, WDatabase};
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    /// Whether a password change is supported
    password_change_supported: bool,
}

/// Check if a password change is supported
pub async fn supports_password_change(
    config: WConfig,
    database: WDatabase,
    _: Auth,
) -> web::Json<Response> {
    let provider = CombinedAuthorizationProvider::new(&config, &database);
    web::Json(Response {
        password_change_supported: provider.supports_password_change(),
    })
}

use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::response_types::Empty;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::{WConfig, WDatabase};
use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    /// The new name
    new_name: String,
}

/// Change the name of the current user
///
/// # Errors
///
/// - If the operation fails
/// - If the operation is not supported
pub async fn change_name(
    mut auth: Auth,
    payload: web::Json<Request>,
    config: WConfig,
    database: WDatabase,
) -> WebResult<Empty> {
    // Check for support
    let provider = CombinedAuthorizationProvider::new(&config, &database);
    if !provider.supports_name_change() {
        return Err(WebErrorKind::Unsupported.into());
    }

    // Espo does not support name changes
    // Update it in the database

    auth.user.set_name(&database, &payload.new_name).await?;

    Ok(Empty)
}

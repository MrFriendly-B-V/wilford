use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::{auth_error_to_web_error, WConfig, WDatabase};
use actix_web::web;
use database::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    /// The name of the new user
    name: String,
    /// The e-mail address of the new user
    email: String,
    /// The password of the new user
    password: String,
}

#[derive(Serialize)]
pub struct Response {
    /// The ID of the created user
    user_id: String,
}

/// Register a new user.
///
/// # Errors
///
/// - If the provider does not support registering new users
/// - If the provided e-mail address already exists
/// - If the underlying operation fails
pub async fn register(
    payload: web::Json<Request>,
    config: WConfig,
    database: WDatabase,
) -> WebResult<web::Json<Response>> {
    let provider = CombinedAuthorizationProvider::new(&config, &database);
    if !provider.supports_registration() {
        return Err(WebErrorKind::Unsupported.into());
    }

    let payload = payload.into_inner();

    let first_user = User::count(&database).await? == 0;

    // Create the user.
    // `unwrap_left` is safe because we will never get a `TotpRequired` error here.
    let new_user = auth_error_to_web_error(
        provider
            .register_user(&payload.name, &payload.email, &payload.password, first_user)
            .await,
    )?
    .unwrap_left();

    // TODO registration email and email verification

    Ok(web::Json(Response {
        user_id: new_user.id,
    }))
}

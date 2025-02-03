use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::mail::WilfordMailer;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::user::email_verify_link;
use crate::routes::{auth_error_to_web_error, WConfig, WDatabase};
use actix_web::web;
use database::user::{Locale, User};
use mailer::{VerifyEmailData, VerifyEmailEmail};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize)]
pub struct Request {
    /// The name of the new user
    name: String,
    /// The e-mail address of the new user
    email: String,
    /// The password of the new user
    password: String,
    /// The preferred locale of the new user
    locale: Locale,
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
            .register_user(
                &payload.name,
                &payload.email,
                &payload.password,
                first_user,
                payload.locale,
            )
            .await,
    )?
    .unwrap_left();

    match (&new_user.email_verification, &config.email) {
        (Some(verification), Some(email_config)) => {
            WilfordMailer::new(email_config)
                .send_email(
                    &new_user.email,
                    VerifyEmailEmail,
                    &VerifyEmailData {
                        name: new_user.name,
                        email_verify_link: email_verify_link(&config, verification),
                    },
                    payload.locale,
                )
                .await?;
        }
        (Some(verification), None) => {
            info!("Email configuration not set");
            info!(
                "Please verify the email of the new user here: {}",
                email_verify_link(&config, verification)
            );
        }
        _ => {}
    }

    Ok(web::Json(Response {
        user_id: new_user.id,
    }))
}

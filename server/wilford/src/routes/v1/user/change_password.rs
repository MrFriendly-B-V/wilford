use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::mail::WilfordMailer;
use crate::response_types::Empty;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::{auth_error_to_web_error, WConfig, WDatabase};
use actix_web::web;
use database::user::Locale;
use mailer::{PasswordChangedData, PasswordChangedMail};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    /// The old password
    old_password: String,
    /// The new password
    new_password: String,
}

/// Change the password of the authorized user.
///
/// # Errors
///
/// - If the provider does not support changing passwords
/// - If the provided `old_password` does not match the stored password
/// - If the change operation fails
pub async fn change_password(
    auth: Auth,
    payload: web::Json<Request>,
    config: WConfig,
    database: WDatabase,
) -> WebResult<Empty> {
    let provider = CombinedAuthorizationProvider::new(&config, &database);
    if !provider.supports_password_change() {
        return Err(WebErrorKind::Unsupported.into());
    }

    // Check the old password is correct
    auth_error_to_web_error(
        provider
            .validate_credentials(&auth.user.email, &payload.old_password, None)
            .await,
    )?;

    // Set new password
    auth_error_to_web_error(
        provider
            .set_password(&auth.user.user_id, &payload.new_password, false)
            .await,
    )?;

    // Inform user of password change via email
    if let Some(email_cfg) = &config.email {
        WilfordMailer::new(email_cfg)
            .send_email(
                &auth.user.email,
                PasswordChangedMail,
                &PasswordChangedData {
                    name: auth.user.name,
                },
                Locale::En,
            )
            .await?;
    }

    Ok(Empty)
}

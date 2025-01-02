use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::mail::WilfordMailer;
use crate::response_types::Empty;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::{auth_error_to_web_error, WConfig, WDatabase};
use actix_web::web;
use mailer::{EmailChangedData, EmailChangedMail, Locale};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    new_email: String,
    password: String,
}

/// Change email address of the user
///
/// # Errors
/// - The operation is not supported
/// - The provided password is invalid
/// - The operation fails
pub async fn change_email(
    auth: Auth,
    payload: web::Json<Request>,
    config: WConfig,
    database: WDatabase,
) -> WebResult<Empty> {
    // Check for support
    let provider = CombinedAuthorizationProvider::new(&config, &database);
    if !provider.supports_email_change() {
        return Err(WebErrorKind::Unsupported.into());
    }

    // Validate password
    auth_error_to_web_error(
        provider
            .validate_credentials(&auth.user.email, &payload.password, None)
            .await,
    )?;

    // Change email
    auth_error_to_web_error(provider.set_email(&auth.user_id, &payload.new_email).await)?;

    // Send email
    if let Some(email_cfg) = &config.email {
        // Old email
        WilfordMailer::new(email_cfg)
            .send_email(
                &auth.user.email,
                EmailChangedMail,
                &EmailChangedData {
                    name: auth.user.name,
                },
                Locale::En,
            )
            .await?;

        // TODO: Verifaction email to new email
    }

    Ok(Empty)
}

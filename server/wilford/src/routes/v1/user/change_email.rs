use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::mail::WilfordMailer;
use crate::response_types::Empty;
use crate::routes::auth::Auth;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::v1::user::email_verify_link;
use crate::routes::{auth_error_to_web_error, WConfig, WDatabase};
use actix_web::web;
use mailer::{VerifyEmailData, VerifyEmailEmail};
use serde::Deserialize;
use tracing::info;

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

    let payload = payload.into_inner();

    // Validate password
    auth_error_to_web_error(
        provider
            .validate_credentials(&auth.user.email, &payload.password, None)
            .await,
    )?;

    let verification = auth.user.update_email(&database, payload.new_email).await?;

    if let Some(email_config) = &config.email {
        WilfordMailer::new(email_config)
            .send_email(
                &auth.user.email,
                VerifyEmailEmail,
                &VerifyEmailData {
                    name: auth.user.name,
                    email_verify_link: email_verify_link(&config, &verification),
                },
                auth.user.locale,
            )
            .await?;
    } else {
        info!("No email configuration provided");
        info!(
            "User email address pending for verification. Use the following URL to verify it: {}",
            email_verify_link(&config, &verification)
        );
    }

    Ok(Empty)
}

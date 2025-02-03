use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::mail::WilfordMailer;
use crate::response_types::Empty;
use crate::routes::error::{WebError, WebErrorKind, WebResult};
use crate::routes::{WConfig, WDatabase};
use actix_web::web;
use database::user::{Locale, SetEmailAddressError, User};
use mailer::{EmailChangedData, EmailChangedMail};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    verification_code: String,
    user_id: String,
}

/// Verify the email address of the user
///
/// # Errors
///
/// - If the authorization provider does not support email changes
/// - If the user does not exist
/// - If the verification code does not exist
/// - If an underlying operation fails
pub async fn verify_email(
    database: WDatabase,
    config: WConfig,
    query: web::Query<Request>,
) -> WebResult<Empty> {
    // Check for support
    let auth = CombinedAuthorizationProvider::new(&config, &database);
    if !auth.supports_email_change() {
        return Err(WebErrorKind::BadRequest.into());
    }

    // Fetch the user
    let mut user = User::get_by_id(&database, &query.user_id)
        .await?
        .ok_or(WebError::from(WebErrorKind::NotFound))?;

    // Fetch the verification
    let verification = user
        .get_address_by_verification_code(&query.verification_code, &database)
        .await?
        .ok_or(WebError::from(WebErrorKind::NotFound))?;

    // Mark the email verified
    user.set_email_verified(&database, &verification.address, true)
        .await?;

    // Remove old verification code
    user.remove_email_verifcation_code(
        &database,
        &verification.address,
        &verification.verification_code,
    )
    .await?;

    // Finally, update the address
    user.set_email(&database, &verification.address)
        .await
        .map_err(|e| match e {
            SetEmailAddressError::NoEmail => WebError::from(WebErrorKind::InvalidInternalState),
            SetEmailAddressError::Sqlx(e) => e.into(),
        })?;

    // Send email
    if let Some(email_cfg) = &config.email {
        // Old email
        WilfordMailer::new(email_cfg)
            .send_email(
                &user.email,
                EmailChangedMail,
                &EmailChangedData { name: user.name },
                Locale::En,
            )
            .await?;
    }

    // The address is now updated

    Ok(Empty)
}

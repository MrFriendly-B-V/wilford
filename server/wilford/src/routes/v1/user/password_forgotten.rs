use crate::authorization::combined::CombinedAuthorizationProvider;
use crate::authorization::AuthorizationProvider;
use crate::mail::WilfordMailer;
use crate::response_types::Empty;
use crate::routes::error::{WebErrorKind, WebResult};
use crate::routes::{auth_error_to_web_error, WConfig, WDatabase};
use actix_web::web;
use database::user::{Locale, User};
use mailer::{PasswordForgottenData, PasswordForgottenMail};
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
}

pub async fn password_forgotten(
    config: WConfig,
    database: WDatabase,
    payload: web::Json<Request>,
) -> WebResult<Empty> {
    // Fetch the user
    let user = match User::get_by_email(&database, &payload.email).await? {
        Some(user) => user,
        None => return Ok(Empty),
    };

    let provider = CombinedAuthorizationProvider::new(&config, &database);
    if !provider.supports_password_change() {
        return Err(WebErrorKind::Unsupported.into());
    }

    // Update the password in the database
    let tmp_password = tmp_password();
    auth_error_to_web_error(
        provider
            .set_password(&user.user_id, &tmp_password, true)
            .await,
    )?;

    // Email the user with their temporary password
    if let Some(email_cfg) = &config.email {
        WilfordMailer::new(email_cfg)
            .send_email(
                &user.email,
                PasswordForgottenMail,
                &PasswordForgottenData {
                    name: user.name,
                    temporary_password: tmp_password,
                },
                Locale::Nl,
            )
            .await?;
    }

    Ok(Empty)
}

/// Generate a temporary password
fn tmp_password() -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

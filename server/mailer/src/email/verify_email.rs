use crate::Mailable;
use database::user::Locale;
use serde::Serialize;

pub struct VerifyEmailEmail;

#[derive(Serialize)]
pub struct VerifyEmailData {
    pub name: String,
    pub email_verify_link: String,
}

impl Mailable for VerifyEmailEmail {
    type Data = VerifyEmailData;

    fn template_name() -> &'static str {
        "verify_email"
    }

    fn subject(locale: &Locale) -> &'static str {
        match locale {
            Locale::Nl => "Email verificatie",
            Locale::En => "Email verification",
        }
    }
}

use crate::email::Mailable;
use database::user::Locale;
use serde::Serialize;

pub struct PasswordChangedMail;

#[derive(Serialize)]
pub struct PasswordChangedData {
    pub name: String,
}

impl Mailable for PasswordChangedMail {
    type Data = PasswordChangedData;

    fn template_name() -> &'static str {
        "password_changed"
    }

    fn subject(locale: &Locale) -> &'static str {
        match locale {
            Locale::En => "Your password was changed",
            Locale::Nl => "Je wachtwoord is gewijzigd",
        }
    }
}

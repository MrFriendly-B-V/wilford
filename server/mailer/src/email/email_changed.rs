use crate::email::Mailable;
use database::user::Locale;
use serde::Serialize;

pub struct EmailChangedMail;

#[derive(Serialize)]
pub struct EmailChangedData {
    pub name: String,
}

impl Mailable for EmailChangedMail {
    type Data = EmailChangedData;

    fn template_name() -> &'static str {
        "email_changed"
    }

    fn subject(locale: &Locale) -> &'static str {
        match locale {
            Locale::En => "Your email address was changed",
            Locale::Nl => "Je email adres is gewijzigd",
        }
    }
}

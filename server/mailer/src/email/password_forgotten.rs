use crate::email::{Locale, Mailable};
use serde::Serialize;

pub struct PasswordForgottenEmail;

#[derive(Serialize)]
pub struct PasswordForgottenData {
    pub name: String,
    pub temporary_password: String,
}

impl Mailable for PasswordForgottenEmail {
    type Data = PasswordForgottenData;

    fn template_name() -> &'static str {
        "password_forgotten"
    }

    fn subject(locale: &Locale) -> &'static str {
        match locale {
            Locale::Nl => "Tijdelijk wachtwoord",
            Locale::En => "Temporary password",
        }
    }
}

#[cfg(test)]
mod test {
    use crate::email::password_forgotten::{PasswordForgottenData, PasswordForgottenEmail};
    use crate::email::{Locale, Mailable};
    use crate::test::{banner_partial, connection};

    #[tokio::test]
    async fn password_forgotten() {
        PasswordForgottenEmail::send(
            &mut connection().await,
            "t.debruijn@array21.dev",
            "t.debruijn@array21.dev",
            &PasswordForgottenData {
                name: "Tobias".to_string(),
                temporary_password: "foobar".to_string(),
            },
            Locale::Nl,
            vec![banner_partial()],
        )
        .await
        .unwrap();
    }
}

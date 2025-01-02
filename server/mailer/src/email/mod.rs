mod email_changed;
mod password_changed;
mod password_forgotten;

pub use email_changed::*;
pub use password_changed::*;
pub use password_forgotten::*;

use crate::error::Result;
use crate::mailer::Mailer;
use crate::{HbsTemplate, Locale};
use lettre::transport::smtp::client::AsyncSmtpConnection;
use serde::Serialize;
use std::future::Future;

/// An email that can be sent.
///
/// This should be implemented for every type of email that should be sent. E.g.:
/// - Pasword changed
/// - Password forgotten
///
/// When implementing, you only have to provide the associated type, `Data`,
/// and implement the functions `template_name` and `subject`. The rest will be handled for you
/// automatically. Though you can change the behaviour by overriding the `send` function.
pub trait Mailable {
    /// The data available for the template to render.
    type Data: Serialize + Send + Sync;

    /// Send an email.
    /// Extra partials can be used for runtime defined partials, e.g. for a banner logo.
    ///
    /// # Errors
    /// - If the `to` or `from` addresses are invalid
    /// - If the body could not be rendered
    /// - If the email could not be sent
    /// - The template does not exist
    // While `async` trait functions are a thing, the compiler discourages it,
    // when using the automatic desugaring (using the `async` keyword), we cannot specify
    // that the future is Send + Sync, which makes life harder for the callee.
    fn send(
        connection: &mut AsyncSmtpConnection,
        to: &str,
        from: &str,
        data: &Self::Data,
        locale: Locale,
        extra_partials: Vec<HbsTemplate>,
    ) -> impl Future<Output = Result<()>> + Send + Sync {
        async {
            Mailer::send(
                connection,
                to,
                from,
                Self::subject(&locale),
                data,
                Self::template_name(),
                locale,
                extra_partials,
            )
            .await
        }
    }

    /// The name of the template used by this E-mail
    fn template_name() -> &'static str;

    /// The subject of the email in the correct locale
    fn subject(locale: &Locale) -> &'static str;
}

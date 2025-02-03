use crate::locale::localize_template_name;
use crate::template::{HbsTemplate, TemplateEngine};
use database::user::Locale;
use lettre::message::{Mailbox, MessageBuilder, SinglePart};
use lettre::transport::smtp::client::AsyncSmtpConnection;
use lettre::Message;
use serde::Serialize;
use std::str::FromStr;

pub(crate) struct Mailer;

impl Mailer {
    /// Send an email
    ///
    /// # Errors
    /// - If the `to` or `from` addresses are invalid
    /// - If the body could not be rendered
    /// - If the email could not be sent
    pub async fn send<S: Serialize>(
        connection: &mut AsyncSmtpConnection,
        to: &str,
        from: &str,
        subject: &str,
        data: &S,
        template_name: &str,
        locale: Locale,
        extra_partials: Vec<HbsTemplate>,
    ) -> crate::error::Result<()> {
        // Render the body
        let handlebars = TemplateEngine::new(extra_partials)?;
        let body_html = handlebars.render(&localize_template_name(&locale, template_name), data)?;

        // Create the message
        let msg =
            Self::prepare_message(to, from, subject)?.singlepart(SinglePart::html(body_html))?;

        // Send the message
        connection.send(msg.envelope(), &msg.formatted()).await?;

        Ok(())
    }

    /// Create the message with a sender, recipient and subject
    ///
    /// # Erorrs
    /// - If the `to` or `from` addresses are invalid
    fn prepare_message(
        to: &str,
        from: &str,
        subject: &str,
    ) -> crate::error::Result<MessageBuilder> {
        let to = Mailbox::from_str(to)?;
        let from = Mailbox::from_str(from)?;

        let msg = Message::builder().to(to).from(from).subject(subject);

        Ok(msg)
    }
}

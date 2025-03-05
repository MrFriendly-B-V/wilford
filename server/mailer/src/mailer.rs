use crate::locale::localize_template_name;
use crate::template::{HbsTemplate, TemplateEngine};
use database::user::Locale;
use lettre::message::{Mailbox, MessageBuilder, SinglePart};
use lettre::transport::smtp::client::AsyncSmtpConnection;
use lettre::Message;
use serde::Serialize;
use std::str::FromStr;

pub(crate) struct Mailer;

pub struct SendMail<'a, S: Serialize> {
    pub to: &'a str,
    pub from: &'a str,
    pub subject: &'a str,
    pub data: &'a S,
}

impl Mailer {
    /// Send an email
    ///
    /// # Errors
    /// - If the `to` or `from` addresses are invalid
    /// - If the body could not be rendered
    /// - If the email could not be sent
    pub async fn send<S: Serialize>(
        connection: &mut AsyncSmtpConnection,
        send_mail: SendMail<'_, S>,
        template_name: &str,
        locale: Locale,
        extra_partials: Vec<HbsTemplate>,
    ) -> crate::error::Result<()> {
        // Render the body
        let handlebars = TemplateEngine::new(extra_partials)?;
        let body_html = handlebars.into_inner().render(
            &localize_template_name(&locale, template_name),
            send_mail.data,
        )?;

        // Create the message
        let msg = Self::prepare_message(send_mail.to, send_mail.from, send_mail.subject)?
            .singlepart(SinglePart::html(body_html))?;

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

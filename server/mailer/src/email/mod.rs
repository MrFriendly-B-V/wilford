mod password_forgotten;

pub use password_forgotten::*;
use std::future::Future;

use crate::error::Result;
use handlebars::Handlebars;
use include_directory::{include_directory, Dir};
use lettre::message::{Mailbox, MessageBuilder, SinglePart};
use lettre::transport::smtp::client::AsyncSmtpConnection;
use lettre::Message;
use serde::Serialize;
use std::str::FromStr;

/// Contents of the `partials` directory.
const PARTIALS: Dir<'_> = include_directory!("mailer/partials");
/// Contents of the `templates` directory
const TEMPLATES: Dir<'_> = include_directory!("mailer/templates/");

/// Language of the email
pub enum Locale {
    Nl,
    En,
}

/// A handlebars template
pub struct HbsTemplate {
    /// The name of the template
    pub name: String,
    /// The value of the template in Handlebars
    pub content: String,
}

/// An email that can be sent.
pub trait Mailable {
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
    // when using the automatic desugaring (using the `async` keyword, we cannot specify
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

struct Mailer;

impl Mailer {
    /// Send an email
    ///
    /// # Errors
    /// - If the `to` or `from` addresses are invalid
    /// - If the body could not be rendered
    /// - If the email could not be sent
    async fn send<S: Serialize>(
        connection: &mut AsyncSmtpConnection,
        to: &str,
        from: &str,
        subject: &str,
        data: &S,
        template_name: &str,
        locale: Locale,
        extra_partials: Vec<HbsTemplate>,
    ) -> Result<()> {
        // Render the body
        let handlebars = Self::new_handlebars(extra_partials)?;
        let body_html =
            handlebars.render(&Self::format_locale_name(template_name, locale), data)?;

        // Create the message
        let msg =
            Self::prepare_message(to, from, subject)?.singlepart(SinglePart::html(body_html))?;

        // Send the message
        connection.send(msg.envelope(), &msg.formatted()).await?;

        Ok(())
    }

    /// Format the name of the template based on the locale.
    /// E.g. the template name `foo` becomes `foo.nl` if the locale is [Locale::Nl].
    fn format_locale_name(template_name: &str, locale: Locale) -> String {
        let locale = match locale {
            Locale::Nl => "nl",
            Locale::En => "en",
        };

        format!("{template_name}.{locale}")
    }

    /// Create the message with a sender, recipient and subject
    ///
    /// # Erorrs
    /// - If the `to` or `from` addresses are invalid
    fn prepare_message(to: &str, from: &str, subject: &str) -> Result<MessageBuilder> {
        let to = Mailbox::from_str(to)?;
        let from = Mailbox::from_str(from)?;

        let msg = Message::builder().to(to).from(from).subject(subject);

        Ok(msg)
    }

    /// Set up the Handlebars engine.
    ///
    /// # Errors
    /// - If a partial is invalid.
    /// - If a template is invalid.
    fn new_handlebars(extra_partials: Vec<HbsTemplate>) -> Result<Handlebars<'static>> {
        let mut handlebars = Handlebars::new();
        handlebars.set_dev_mode(is_dev_mode());
        handlebars.set_strict_mode(true);

        // Register partials
        for template in Self::partials() {
            handlebars.register_partial(&template.name, template.content)?;
        }

        // Register the extra partials (runtime defined)
        for template in extra_partials {
            handlebars.register_partial(&template.name, template.content)?;
        }

        // Register templates
        for template in Self::templates() {
            handlebars.register_template_string(&template.name, template.content)?;
        }

        Ok(handlebars)
    }

    /// Get all partials stored in the binary.
    /// Returns a tuple of (name, content).
    fn partials() -> Vec<HbsTemplate> {
        Self::get_embed(PARTIALS)
    }

    /// Get all templates stored in the binary.
    /// Returns a tuple of (name, content).
    fn templates() -> Vec<HbsTemplate> {
        Self::get_embed(TEMPLATES)
    }

    /// Get all files in an embedded direcotry.
    /// Returns a tuple of (name, content).
    fn get_embed(embed: Dir<'_>) -> Vec<HbsTemplate> {
        embed
            .files()
            .into_iter()
            .map(|f| {
                let name = f
                    .path()
                    .file_name()
                    .map(|fname| fname.to_str())
                    .flatten()
                    // Split by .
                    .map(|fname| fname.split(".").collect::<Vec<_>>())
                    // Keep all but the last element
                    .map(|parts| all_but_last(parts.into_iter()))
                    // Re-join string
                    .map(|parts: Vec<&str>| parts.join("."))
                    .map(|fname| fname.to_string());

                let contents = f.contents_utf8().map(|cnt| cnt.to_string());

                match (name, contents) {
                    (Some(n), Some(c)) => Some(HbsTemplate {
                        name: n,
                        content: c,
                    }),
                    _ => None,
                }
            })
            .filter_map(|hbs_template| hbs_template)
            .collect()
    }
}

/// Whether the program is compiled in debug mode
fn is_dev_mode() -> bool {
    cfg!(debug_assertions)
}

/// Keep all elements except the last element
fn all_but_last<T, I, B>(iter: I) -> B
where
    B: FromIterator<T>,
    I: DoubleEndedIterator<Item = T> + ExactSizeIterator<Item = T>,
{
    iter.rev().skip(1).rev().collect()
}

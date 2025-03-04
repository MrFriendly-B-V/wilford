use crate::config::EmailConfig;
use database::user::Locale;
use mailer::{HbsTemplate, Mailable};
use std::path::Path;
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncReadExt;

#[derive(Debug, Error)]
pub enum MailerError {
    #[error(transparent)]
    Email(#[from] mailer::MailerError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub struct WilfordMailer<'a> {
    config: &'a EmailConfig,
}

impl<'a> WilfordMailer<'a> {
    pub fn new(config: &'a EmailConfig) -> Self {
        Self { config }
    }

    /// Send an email.
    ///
    /// # Errors
    ///
    /// - If the email could not be sent.
    /// - If opening one of the configured Handlebars templates fails.
    pub async fn send_email<S, M>(
        &self,
        to: S,
        // We don't need the actual value, just the type,
        // but passing it as a parameter makes the code cleaner
        // at the call site.
        _: M,
        mailer_data: &M::Data,
        locale: Locale,
    ) -> Result<(), MailerError>
    where
        S: AsRef<str>,
        M: Mailable,
    {
        // Establish the SMTP connection
        let ipv4 = mailer::net::get_local_v4()
            .await
            .map_err(|e| mailer::MailerError::from(e))?;
        let mut conn =
            mailer::net::get_connection(ipv4, &self.config.smtp, &self.get_ehlo_domain()).await?;

        // Send the email
        M::send(
            &mut conn,
            to.as_ref(),
            &self.config.from,
            mailer_data,
            locale,
            self.load_partials().await?,
        )
        .await?;

        Ok(())
    }

    /// Get the EHLO domain from the configured `From` value.
    fn get_ehlo_domain(&self) -> String {
        // Original format: `Name <email@domain>`

        // Trim to `domain>`
        let domain = self.config.from.split("@").collect::<Vec<_>>()[1];
        // Remove trailing `>`
        domain.replace(">", "")
    }

    /// Load all configured partials.
    ///
    /// # Errors
    ///
    /// If an IO error occurs.
    async fn load_partials(&self) -> Result<Vec<HbsTemplate>, MailerError> {
        Ok(vec![
            Self::load_partial(&self.config.banner_file, "banner").await?,
        ])
    }

    /// Load a file as a partial
    ///
    /// # Errors
    ///
    /// If an IO error occurs.
    async fn load_partial<P: AsRef<Path>>(p: P, name: &str) -> Result<HbsTemplate, MailerError> {
        let mut f = fs::File::open(p).await?;
        let mut content = String::new();
        f.read_to_string(&mut content).await?;

        Ok(HbsTemplate {
            name: name.to_string(),
            content,
        })
    }
}

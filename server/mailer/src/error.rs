use thiserror::Error;

pub type Result<T> = std::result::Result<T, MailerError>;

#[derive(Debug, Error)]
pub enum MailerError {
    #[error("No IPv4 address could be found")]
    NoIpv4,
    #[error("Could not look up addresses: {0}")]
    GetAddr(nix::errno::Errno),
    #[error("Could not connect with SMTP server")]
    SmtpConnect,
    #[error(transparent)]
    Transport(#[from] lettre::transport::smtp::Error),
    #[error(transparent)]
    Template(#[from] handlebars::TemplateError),
    #[error(transparent)]
    Render(#[from] handlebars::RenderError),
    #[error(transparent)]
    Email(#[from] lettre::error::Error),
    #[error(transparent)]
    IpAddress(#[from] crate::ipv4::AddressError),
    #[error(transparent)]
    EmailAddress(#[from] lettre::address::AddressError),
}

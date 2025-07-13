use crate::error::{MailerError, Result};
use lettre::transport::smtp::client::{AsyncSmtpConnection, TlsParameters};
use lettre::transport::smtp::extension::ClientId;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tracing::{debug, error, trace};

/// Get an SMTP connection.
///
/// # Errors
/// - If the connection failed.
/// - If establishing the TLS connection failed.
pub async fn get_connection(
    addr: Ipv4Addr,
    smtp_server: &str,
    ehlo_domain: &str,
) -> Result<AsyncSmtpConnection> {
    let client_id = ClientId::Domain(ehlo_domain.to_string());

    trace!("Opening SMTP connection");
    let mut conn = AsyncSmtpConnection::connect_tokio1(
        (smtp_server, 587),
        Some(Duration::from_secs(3)),
        &client_id,
        // We cannot do STARTTLS (which uses port 465, which is blocked by Hetzner), so use port 587
        // Port 587 starts out with regular SMTP commands, after the EHLO we upgrade to STARTTLS
        None,
        Some(IpAddr::V4(addr)),
    )
    .await?;

    // If we can upgrade to STARTLS, do so
    if conn.can_starttls() {
        conn.starttls(
            TlsParameters::new_rustls(smtp_server.to_string())?,
            &client_id,
        )
        .await?;
    }

    trace!("Checking SMTP connection");
    if conn.test_connected().await {
        debug!("SMTP connection OK");
        Ok(conn)
    } else {
        error!("Could not connect to server (SMTP)");
        Err(MailerError::SmtpConnect)
    }
}

//! Email sending library with Handlebars templates
//!
//! # Example
//!
//! ```no_run
//! # use mailer::email::Mailable;
//! # async fn main () {
//! // Get IPV4 address
//! let addr = mailer::ipv4::get_local_v4().await.unwrap();
//! // Establish a connection
//! let mut connection = mailer::conn::get_connection(
//!     addr,
//!     "smtp-relay.gmail.com",
//!     "array21.dev"
//! ).await.unwrap();
//!
//! // Send a password forgotten email
//! mailer::email::PasswordForgottenEmail::send(
//!     &mut connection,
//!     "receiver@array21.dev",
//!     "sender@array21.dev",
//!     &mailer::email::PasswordForgottenData {
//!         name: "Reciever name".to_string(),
//!         temporary_password: "foobarbaz".to_string(),
//!     },
//!     mailer::email::Locale::En,
//!     // You can specify custom Handlebars partials to be used in the templates!
//!     vec![
//!         mailer::email::HbsTemplate {
//!             name: "banner".to_string(),
//!             content: r#"<div class="banner">My custom banner</div>"#.to_string(),
//!         }
//!     ]
//! ).await.unwrap();
//! # }
//! ```

pub mod conn;
pub mod email;
pub mod error;
pub mod ipv4;

#[cfg(test)]
pub(crate) mod test {
    use crate::email::HbsTemplate;
    use lettre::transport::smtp::client::AsyncSmtpConnection;
    use std::net::Ipv4Addr;

    async fn ipv4() -> Ipv4Addr {
        crate::ipv4::get_local_v4().await.unwrap()
    }

    pub async fn connection() -> AsyncSmtpConnection {
        crate::conn::get_connection(ipv4().await, "smtp-relay.gmail.com", "array21.dev")
            .await
            .unwrap()
    }

    pub fn banner_partial() -> HbsTemplate {
        HbsTemplate {
            name: "banner".to_string(),
            content: r#"<div class="banner">My custom banner</div>"#.to_string(),
        }
    }
}

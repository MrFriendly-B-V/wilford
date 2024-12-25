mod local_provider;

use std::error::Error;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthorizationError<E: Error + Debug> {
    #[error("Invalid credentials provided")]
    InvalidCredentials,
    #[error("Two-factor authentication required")]
    TotpNeeded,
    #[error("Unsupported operation")]
    UnsupportedOperation,
    #[error(transparent)]
    Other(#[from] E),
}

/// Information about the authorized user
#[derive(Debug)]
pub struct UserInformation {
    /// The ID of the user. This ID should be used everywhere else,
    /// the authorization provider is the ultimate source of truth for which users
    /// can use the system.
    pub id: String,
    /// Whether the user is a global administrator.
    /// If true, all scope checks should be ignored.
    pub is_admin: bool,
    /// The name of the user.
    pub name: String,
    /// The email address of the user.
    pub email: String,
}

pub trait AuthorizationProvider {
    /// Error that can be returned by the authorization provider
    type Error: std::error::Error;

    /// Validate the given credentials.
    /// If the credentials are correct the associated user will be returned
    ///
    /// # Errors
    /// - If the credentials are invalid
    /// - If an underlying operaiton fails
    /// - If two-factor authentication is required
    async fn validate_credentials(
        &self,
        username: &str,
        password: &str,
        totp_code: Option<&str>,
    ) -> Result<UserInformation, AuthorizationError<Self::Error>>;

    /// Whether the authorization provider supports changing the user's password.
    fn supports_password_change(&self) -> bool;

    /// Change the password of the user.
    /// Implementations do not have to support this operation, check this with [Self::supports_password_change].
    ///
    /// # Errors
    /// - If the operation is not supported
    /// - If an underlying operation fails
    /// - If the user does not exist
    async fn set_password(
        &self,
        user_id: &str,
        new_password: &str,
    ) -> Result<(), AuthorizationError<Self::Error>>;
}

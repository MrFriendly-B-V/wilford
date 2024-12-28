pub mod combined;
pub mod espo;
pub mod local_provider;

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
    #[error("Already exists")]
    AlreadyExists,
    #[error(transparent)]
    Other(#[from] E),
}

impl<E: Error + Debug> AuthorizationError<E> {
    /// Convert an authorization error with a generic inner type into an authorization error with a different `Other` type,
    /// if `E` can be turned into a `T`.
    // This is not possible with a `impl<T, U> From<AuthorizationError<T>> for AuthorizationError<U> where U: From<T>` as it overlaps
    // with the standard library in the case that `T = U`. We know it won't, but we can't yet tell the compiler that.
    fn convert<T: Error + Debug + From<E>>(self) -> AuthorizationError<T> {
        match self {
            AuthorizationError::Other(e) => AuthorizationError::Other(T::from(e)),
            AuthorizationError::InvalidCredentials => AuthorizationError::InvalidCredentials,
            AuthorizationError::AlreadyExists => AuthorizationError::AlreadyExists,
            AuthorizationError::TotpNeeded => AuthorizationError::TotpNeeded,
            AuthorizationError::UnsupportedOperation => AuthorizationError::UnsupportedOperation,
        }
    }
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
    #[allow(unused)]
    pub name: String,
    /// The email address of the user.
    #[allow(unused)]
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

    /// Whether the authorization provider supports registering new users.
    fn supports_registration(&self) -> bool;

    /// Register a new user with the authorization provider.
    /// Implementations do not have to support this operation, check this with [Self::supports_registration].
    ///
    /// # Errors
    /// - If the operation is not supports
    /// - If the underlying operation fails
    /// - If a user with the given e-mail address already exists
    async fn register_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> Result<UserInformation, AuthorizationError<Self::Error>>;
}

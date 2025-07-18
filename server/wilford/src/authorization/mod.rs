pub mod combined;
pub mod espo;
pub mod local_provider;

use database::user::{Locale, UserEmailVerification};
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
    #[allow(unused)]
    /// The verification required to verify the user's email, if required
    pub email_verification: Option<UserEmailVerification>,
}

pub struct CredentialsValidationResult {
    /// Information about the user
    pub user_information: UserInformation,
    /// Whether the password of the user needs to be changed.
    /// The frontend will inform the user of this when they navigate to the `/me` page,
    /// this field is also provided via the `/user/info` endpoint.
    #[allow(unused)]
    pub require_password_change: bool,
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
    ) -> Result<CredentialsValidationResult, AuthorizationError<Self::Error>>;

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
        require_change: bool,
    ) -> Result<(), AuthorizationError<Self::Error>>;

    /// Whether the authorization provider supports registering new users.
    fn supports_registration(&self) -> bool;

    /// Register a new user with the authorization provider.
    /// Implementations do not have to support this operation, check this with [Self::supports_registration].
    ///
    /// # Errors
    /// - If the operation is not supported
    /// - If the underlying operation fails
    /// - If a user with the given e-mail address already exists
    async fn register_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
        is_admin: bool,
        locale: Locale,
    ) -> Result<UserInformation, AuthorizationError<Self::Error>>;

    /// Whether the provider supports changing the email address of the user.
    fn supports_email_change(&self) -> bool;

    /// Change the email address of the user.
    /// Implementations do not have to support this operation, check this with [Self::supports_email_change]
    ///
    /// # Errors
    /// - If the operation is not supported
    /// - IF the underlying operation fails
    async fn set_email(
        &mut self,
        user_id: &str,
        new_email: &str,
    ) -> Result<(), AuthorizationError<Self::Error>>;

    fn supports_name_change(&self) -> bool;
}

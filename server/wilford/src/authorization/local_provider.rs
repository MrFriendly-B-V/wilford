use crate::authorization::{AuthorizationError, AuthorizationProvider, UserInformation};
use bcrypt::{hash_with_result, verify, Version};
use database::driver::Database;
use database::user::User;
use rand::Rng;
use tap::TapOptional;
use thiserror::Error;
use tracing::{instrument, warn};

/// Credential provider utilizing the local database
pub struct LocalAuthorizationProvider<'a> {
    driver: &'a Database,
}

#[derive(Debug, Error)]
pub enum LocalAuthorizationProviderError {
    #[error(transparent)]
    Database(#[from] database::driver::Error),
    #[error(transparent)]
    Hashing(#[from] bcrypt::BcryptError),
}

impl<'a> LocalAuthorizationProvider<'a> {
    /// Create a new provider.
    pub fn new(driver: &'a Database) -> Self {
        Self { driver }
    }
}

impl<'a> AuthorizationProvider for LocalAuthorizationProvider<'a> {
    type Error = LocalAuthorizationProviderError;

    #[instrument(skip(self, password))]
    async fn validate_credentials(
        &self,
        username: &str,
        password: &str,
        _: Option<&str>,
    ) -> Result<UserInformation, AuthorizationError<Self::Error>> {
        // Fetch the user
        let user = User::get_by_email(self.driver, username)
            .await
            .map_err(Self::Error::from)?
            .ok_or(AuthorizationError::InvalidCredentials)?;

        // Fetch the hash
        let stored_hash = user
            .get_password_hash(self.driver)
            .await
            .map_err(Self::Error::from)?
            .tap_none(|| {
                warn!("No credentials stored for user with email {username}, but user does exist")
            })
            .ok_or(AuthorizationError::InvalidCredentials)?;

        // Use bcrypt to verify the hash is correct
        let ok = verify(password, &stored_hash).map_err(Self::Error::from)?;

        if ok {
            Ok(UserInformation {
                id: user.user_id,
                email: user.email,
                name: user.name,
                is_admin: user.is_admin,
            })
        } else {
            Err(AuthorizationError::InvalidCredentials)
        }
    }

    fn supports_password_change(&self) -> bool {
        true
    }

    #[instrument(skip(self, new_password))]
    async fn set_password(
        &self,
        user_id: &str,
        new_password: &str,
    ) -> Result<(), AuthorizationError<Self::Error>> {
        // Fetch the user
        let user = User::get_by_id(self.driver, user_id)
            .await
            .map_err(Self::Error::from)?
            .ok_or(AuthorizationError::InvalidCredentials)?;

        // Generate the new password hash.
        let new_password = hash_password(&new_password)?;

        // Finally, update the database
        user.set_password_hash(self.driver, new_password)
            .await
            .map_err(Self::Error::from)?;

        Ok(())
    }

    fn supports_registration(&self) -> bool {
        true
    }

    #[instrument(skip(self, password))]
    async fn register_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> Result<UserInformation, AuthorizationError<Self::Error>> {
        // Check that the user does not already exist
        User::get_by_email(self.driver, email)
            .await
            .map_err(Self::Error::from)?
            .map(|_| Err(AuthorizationError::AlreadyExists))
            .unwrap_or(Ok(()))?;

        let user_id = gen_user_id();

        // Create the user in the database
        let user = User::new(
            &self.driver,
            user_id.clone(),
            name.to_string(),
            email.to_string(),
            is_admin,
        )
        .await
        .map_err(Self::Error::from)?;

        // Hash password
        let password = hash_password(password)?;

        // Set password
        user.set_password_hash(&self.driver, password)
            .await
            .map_err(Self::Error::from)?;

        Ok(UserInformation {
            id: user_id,
            name: name.to_string(),
            email: email.to_string(),
            is_admin,
        })
    }
}

/// Hash the password.
///
/// # Errors
/// If hashing fails
fn hash_password(password: &str) -> Result<String, LocalAuthorizationProviderError> {
    // We are explicit with the format wanted, thus we use `hash_with_result`, rather than
    // `hash`. Although at the moment this block is identical to bcrypt's `hash` function,
    // this could change in the future. That would result in some rather annoying
    // differences in the way we hash.
    hash_with_result(password, bcrypt::DEFAULT_COST)
        .map_err(|e| LocalAuthorizationProviderError::from(e))
        .map(|parts| parts.format_for_version(Version::TwoB))
}

/// Generate a random alphanumeric user ID.
fn gen_user_id() -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect::<String>()
}

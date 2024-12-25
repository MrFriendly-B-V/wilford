use crate::authorization::{AuthorizationError, AuthorizationProvider, UserInformation};
use bcrypt::{hash_with_result, verify, Version};
use database::driver::Database;
use database::user::User;
use tap::TapOptional;
use thiserror::Error;
use tracing::{instrument, warn};

/// Credential provider utilizing the local database
pub struct LocalCredentialsProvider<'a> {
    driver: &'a Database,
}

#[derive(Debug, Error)]
pub enum LocalCredentialsProviderError {
    #[error(transparent)]
    Database(#[from] database::driver::Error),
    #[error(transparent)]
    Hashing(#[from] bcrypt::BcryptError),
}

impl<'a> LocalCredentialsProvider<'a> {
    /// Create a new local credentials provider.
    pub fn new(driver: &'a Database) -> Self {
        Self { driver }
    }
}

impl<'a> AuthorizationProvider for LocalCredentialsProvider<'a> {
    type Error = LocalCredentialsProviderError;

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
            .map_err(|e| Self::Error::from(e))?
            .ok_or(AuthorizationError::InvalidCredentials)?;

        // Fetch the hash
        let stored_hash = user
            .get_password_hash(self.driver)
            .await
            .map_err(|e| Self::Error::from(e))?
            .tap_none(|| {
                warn!("No credentials stored for user with email {username}, but user does exist")
            })
            .ok_or(AuthorizationError::InvalidCredentials)?;

        // Use bcrypt to verify the hash is correct
        let ok = verify(password, &stored_hash).map_err(|e| Self::Error::from(e))?;

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
            .map_err(|e| Self::Error::from(e))?
            .ok_or(AuthorizationError::InvalidCredentials)?;

        // Generate the new password hash.
        // We are explicit with the format wanted, thus we use `hash_with_result`, rather than
        // `hash`. Although at the moment this block is identical to bcrypt's `hash` function,
        // this could change in the future. That would result in some rather annoying
        // differences in the way we hash.
        let new_password = hash_with_result(new_password, bcrypt::DEFAULT_COST)
            .map_err(|e| Self::Error::from(e))?
            .format_for_version(Version::TwoB);

        // Finally, update the database
        user.set_password_hash(self.driver, new_password)
            .await
            .map_err(|e| Self::Error::from(e))?;

        Ok(())
    }
}

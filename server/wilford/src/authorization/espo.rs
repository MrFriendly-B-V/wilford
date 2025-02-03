use crate::authorization::{
    AuthorizationError, AuthorizationProvider, CredentialsValidationResult, UserInformation,
};
use crate::espo::user::{EspoUser, LoginStatus};
use database::driver::Database;
use database::user::{Locale, User};
use espocrm_rs::EspoApiClient;
use thiserror::Error;
use tracing::instrument;

/// Authorization provider utilizing EspoCRM as it's credentials validator and user database.
#[derive(Debug)]
pub struct EspoAuthorizationProvider<'a> {
    database_driver: &'a Database,
    espocrm_client: EspoApiClient,
    host: &'a str,
}

impl<'a> EspoAuthorizationProvider<'a> {
    pub fn new(
        espocrm_client: EspoApiClient,
        host: &'a str,
        database_driver: &'a Database,
    ) -> Self {
        Self {
            espocrm_client,
            host,
            database_driver,
        }
    }
}

#[derive(Debug, Error)]
pub enum EspoAuthorizationProviderError {
    #[error(transparent)]
    Espocrm(#[from] reqwest::Error),
    #[error(transparent)]
    Database(#[from] database::driver::Error),
}

impl<'a> AuthorizationProvider for EspoAuthorizationProvider<'a> {
    type Error = EspoAuthorizationProviderError;

    #[instrument(skip(self, password))]
    async fn validate_credentials(
        &self,
        username: &str,
        password: &str,
        totp_code: Option<&str>,
    ) -> Result<CredentialsValidationResult, AuthorizationError<Self::Error>> {
        // Check the credentials with the EspoCRM instance
        // This will yield the user ID
        let user_id = match EspoUser::try_login(&self.host, username, password, totp_code)
            .await
            .map_err(Self::Error::from)?
        {
            LoginStatus::Ok(user_id) => user_id,
            LoginStatus::SecondStepRequired => return Err(AuthorizationError::TotpNeeded),
            LoginStatus::Err => return Err(AuthorizationError::InvalidCredentials),
        };

        // Fetch user information
        let espo_user = EspoUser::get_by_id(&self.espocrm_client, &user_id)
            .await
            .map_err(|e| AuthorizationError::Other(e.into()))?;

        if !espo_user.is_active {
            return Err(AuthorizationError::InvalidCredentials);
        }

        // Admin status in EspoCRM
        let is_admin = espo_user.user_type.eq("admin");

        // While we now know the user is authorized,
        // we do want to ensure the user is also represented in our database correctly
        let db_user = User::get_by_id(&self.database_driver, &user_id)
            .await
            .map_err(|e| AuthorizationError::Other(e.into()))?;

        if let Some(mut db_user) = db_user {
            // Make sure the database correctly reflects the user's information

            // Admin status
            if db_user.is_admin != is_admin {
                db_user
                    .set_is_admin(&self.database_driver, is_admin)
                    .await
                    .map_err(|e| AuthorizationError::Other(e.into()))?;
            }

            // Name
            if db_user.name.ne(&espo_user.name) {
                db_user
                    .set_name(&self.database_driver, &espo_user.name)
                    .await
                    .map_err(|e| AuthorizationError::Other(e.into()))?;
            }
        } else {
            // Create the user in the database

            User::new(
                self.database_driver,
                user_id,
                espo_user.name.clone(),
                espo_user.email_address.clone(),
                is_admin,
                Locale::Nl,
                false,
            )
            .await
            .map_err(|e| AuthorizationError::Other(e.into()))?;
        }

        Ok(CredentialsValidationResult {
            user_information: UserInformation {
                id: espo_user.id,
                name: espo_user.name,
                email: espo_user.email_address,
                email_verification: None,
                is_admin: espo_user.user_type.eq("admin"),
            },
            require_password_change: false,
        })
    }

    fn supports_password_change(&self) -> bool {
        // Password changes are handled in EspoCRM.

        false
    }

    #[instrument(skip_all)]
    async fn set_password(
        &self,
        _: &str,
        _: &str,
        _: bool,
    ) -> Result<(), AuthorizationError<Self::Error>> {
        Err(AuthorizationError::UnsupportedOperation)
    }

    fn supports_registration(&self) -> bool {
        // Registration of users is managed in EspoCRM
        false
    }

    #[instrument(skip_all)]
    async fn register_user(
        &self,
        _: &str,
        _: &str,
        _: &str,
        _: bool,
        _: Locale,
    ) -> Result<UserInformation, AuthorizationError<Self::Error>> {
        Err(AuthorizationError::UnsupportedOperation)
    }

    fn supports_email_change(&self) -> bool {
        false
    }

    #[instrument(skip_all)]
    async fn set_email(&mut self, _: &str, _: &str) -> Result<(), AuthorizationError<Self::Error>> {
        Err(AuthorizationError::UnsupportedOperation)
    }
}

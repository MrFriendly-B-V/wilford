use crate::authorization::espo::{EspoAuthorizationProvider, EspoAuthorizationProviderError};
use crate::authorization::local_provider::{
    LocalAuthorizationProvider, LocalAuthorizationProviderError,
};
use crate::authorization::{
    AuthorizationError, AuthorizationProvider, CredentialsValidationResult, UserInformation,
};
use crate::config::{AuthorizationProviderType, Config};
use database::driver::Database;
use database::user::Locale;
use espocrm_rs::EspoApiClient;
use std::fmt::Debug;
use thiserror::Error;
use tracing::instrument;

#[derive(Debug, Error)]
pub enum CombinedAuthorizationProviderError {
    #[error(transparent)]
    Local(#[from] LocalAuthorizationProviderError),
    #[error(transparent)]
    EspoCrm(#[from] EspoAuthorizationProviderError),
}

/// Abstraction over all the different authorization providers,
/// providing a single object to work with.
pub enum CombinedAuthorizationProvider<'a> {
    Local(LocalAuthorizationProvider<'a>),
    EspoCrm(EspoAuthorizationProvider<'a>),
}

impl<'a> CombinedAuthorizationProvider<'a> {
    /// Construct a new authorization provider.
    /// The provider backend is chosen dynamically based on the confiuration provided.
    ///
    /// # Panics
    ///
    /// If a backend requiring configuration is configured, but the backend's configuration
    /// is not provided.
    pub fn new(config: &'a Config, database: &'a Database) -> Self {
        match config.authorization_provider {
            AuthorizationProviderType::Local => {
                Self::Local(LocalAuthorizationProvider::new(database))
            }
            AuthorizationProviderType::EspoCrm => {
                if let Some(espo_config) = &config.espo {
                    let client = EspoApiClient::new(&espo_config.host)
                        .set_api_key(&espo_config.api_key)
                        .set_secret_key(&espo_config.secret_key)
                        .build();

                    Self::EspoCrm(EspoAuthorizationProvider::new(
                        client,
                        &espo_config.host,
                        database,
                    ))
                } else {
                    panic!("EspoCrm configured as authorization provider, but no config set for EspoCrm");
                }
            }
        }
    }
}

impl<'a> AuthorizationProvider for CombinedAuthorizationProvider<'a> {
    type Error = CombinedAuthorizationProviderError;

    async fn validate_credentials(
        &self,
        username: &str,
        password: &str,
        totp_code: Option<&str>,
    ) -> Result<CredentialsValidationResult, AuthorizationError<CombinedAuthorizationProviderError>>
    {
        Ok(match self {
            Self::Local(credentials_provider) => credentials_provider
                .validate_credentials(username, password, totp_code)
                .await
                .map_err(AuthorizationError::convert)?,

            Self::EspoCrm(espocrm) => espocrm
                .validate_credentials(username, password, totp_code)
                .await
                .map_err(AuthorizationError::convert)?,
        })
    }

    fn supports_password_change(&self) -> bool {
        match self {
            Self::Local(credentials_provider) => credentials_provider.supports_password_change(),
            Self::EspoCrm(espocrm) => espocrm.supports_password_change(),
        }
    }

    async fn set_password(
        &self,
        user_id: &str,
        new_password: &str,
        require_change: bool,
    ) -> Result<(), AuthorizationError<Self::Error>> {
        match self {
            Self::Local(local) => local
                .set_password(user_id, new_password, require_change)
                .await
                .map_err(AuthorizationError::convert)?,
            Self::EspoCrm(espocrm) => espocrm
                .set_password(user_id, new_password, require_change)
                .await
                .map_err(AuthorizationError::convert)?,
        }

        Ok(())
    }

    fn supports_registration(&self) -> bool {
        match self {
            Self::Local(credentials_provider) => credentials_provider.supports_registration(),
            Self::EspoCrm(espocrm) => espocrm.supports_registration(),
        }
    }

    #[instrument(skip(self, email, password))]
    async fn register_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
        is_admin: bool,
        locale: Locale,
    ) -> Result<UserInformation, AuthorizationError<Self::Error>> {
        Ok(match self {
            Self::Local(credentials_provider) => credentials_provider
                .register_user(name, email, password, is_admin, locale)
                .await
                .map_err(AuthorizationError::convert)?,
            Self::EspoCrm(espocrm) => espocrm
                .register_user(name, email, password, is_admin, locale)
                .await
                .map_err(AuthorizationError::convert)?,
        })
    }

    fn supports_email_change(&self) -> bool {
        match self {
            Self::Local(v) => v.supports_email_change(),
            Self::EspoCrm(v) => v.supports_email_change(),
        }
    }

    async fn set_email(
        &mut self,
        user_id: &str,
        new_email: &str,
    ) -> Result<(), AuthorizationError<Self::Error>> {
        match self {
            Self::Local(v) => v
                .set_email(user_id, new_email)
                .await
                .map_err(AuthorizationError::convert),
            Self::EspoCrm(v) => v
                .set_email(user_id, new_email)
                .await
                .map_err(AuthorizationError::convert),
        }
    }

    fn supports_name_change(&self) -> bool {
        match self {
            Self::Local(credentials_provider) => credentials_provider.supports_name_change(),
            Self::EspoCrm(espocrm) => espocrm.supports_name_change(),
        }
    }
}

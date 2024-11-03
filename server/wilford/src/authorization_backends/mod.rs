use crate::authorization_backends::espocrm::EspoAuthorizationClient;
use thiserror::Error;

pub mod espocrm;

pub enum AuthorizationProvider {
    Espocrm(EspoAuthorizationClient),
}

#[derive(Debug, Error)]
pub enum AuthorizationProviderError {
    #[error(transparent)]
    Espocrm(#[from] reqwest::Error),
}

#[derive(Debug)]
pub struct LoginUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_active: bool,
}

#[derive(Debug)]
pub enum CheckResult {
    Ok(LoginUser),
    TwoFactorRequired,
    Invalid,
}

pub trait AuthorizationBackend {
    type Error: std::error::Error;

    async fn check_credentials(
        &self,
        username: &str,
        password: &str,
        two_factor_code: Option<&str>,
    ) -> Result<CheckResult, Self::Error>;
    async fn get_user(&self, user_id: &str) -> Result<LoginUser, Self::Error>;
}

impl AuthorizationBackend for AuthorizationProvider {
    type Error = AuthorizationProviderError;

    async fn check_credentials(
        &self,
        username: &str,
        password: &str,
        two_factor_code: Option<&str>,
    ) -> Result<CheckResult, Self::Error> {
        match self {
            Self::Espocrm(client) => {
                client
                    .check_credentials(username, password, two_factor_code)
                    .await
            }
        }
        .map_err(|e| e.into())
    }

    async fn get_user(&self, user_id: &str) -> Result<LoginUser, Self::Error> {
        match self {
            Self::Espocrm(client) => client.get_user(user_id).await,
        }
        .map_err(|e| e.into())
    }
}

use color_eyre::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncReadExt;
use tracing::trace;

#[derive(Debug, Deserialize)]
struct EnvConfig {
    /// The path to the JSON configuration file.
    config_path: PathBuf,
}

/* ANCHOR: config */
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Configuration for the basic functioning of the system
    pub http: HttpConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// The authorization provider to use
    pub authorization_provider: AuthorizationProviderType,
    /// EspoCRM configuration.
    /// Required if `authorization_provider` is set to `EspoCrm`
    pub espo: Option<EspoConfig>,
    /// Options for the default OAuth2 client.
    /// This client is used by Wilford itself.
    pub default_client: DefaultClientConfig,
    /// The path to the private key used to sign OIDC ID tokens.
    /// Should be the matching private key of [oidc_public_key]
    pub oidc_signing_key: PathBuf,
    /// The path to the public key used to sign OIDC ID tokens.
    /// Should be the matching public key of [oidc_signing_key]
    pub oidc_public_key: PathBuf,
    /// The issuer of OpenID Connect ID tokens.
    /// E.g. `mrfriendly.nl`.
    pub oidc_issuer: String,
    /// Email configuration.
    /// If this is not set, no emails will be sent,
    /// useful for debugging
    pub email: Option<EmailConfig>,
}

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    /// The SMTP host.
    /// For Gmail this is `smtp-relay.gmail.com`.
    pub smtp: String,
    /// The `From` email address. Use `name <email> syntax.
    /// E.g. `Wilford <no-reply@mrfriendly.nl>`.
    pub from: String,
    /// The path to a Handlebars (`.hbs`) file. This banner
    /// will be included at the top of every email.
    pub banner_file: PathBuf,
}

#[derive(Debug, Deserialize)]
pub enum AuthorizationProviderType {
    /// Use the local database as authorization provider.
    Local,
    /// Use EspoCRM as authorization provider.
    /// Requires further configuration.
    EspoCrm,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    /// The URL at which the frontend's login page can be found.
    pub ui_login_path: String,
    /// The URL at which the frontend's email verification page can be found.
    /// The server will append a `code` and a `user_id` query parameter to the URL
    pub ui_email_verification_path: String,
    /// The URL at which the OAuth2 authorization endpoint can be found.
    /// Should point to the route `/oauth/authorize`.
    pub authorization_endpoint: String,
    /// The URL at which the OAuth2 token endpoint can be found.
    /// Should point to the route `/oauth/token`.
    pub token_endpoint: String,
    /// The URL at which the JWKS document can be found.
    /// Should point to the route `/.well-known/jwks.json`.
    pub jwks_uri_endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct EspoConfig {
    /// The base URI at which EspoCRM can be found.
    /// Should not end with a slash character.
    pub host: String,
    /// The API key of the EspoCRM API client.
    pub api_key: String,
    /// The secret key of the EspoCRM API client.
    pub secret_key: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    /// The MySQL username.
    pub user: String,
    /// The MySQL password.
    pub password: String,
    /// The address at which the MySQL database can be reacher.
    pub host: String,
    /// The MySQL database name.
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct DefaultClientConfig {
    /// The redict URI for the default OAuth2 client used by Wilford itself.
    /// Should be the location at which the frontend's `login-ok` page can be found.
    pub redirect_uri: String,
}
/* ANCHOR_END: config */

impl EnvConfig {
    fn new() -> Result<Self> {
        Ok(envy::from_env()?)
    }
}

impl Config {
    async fn open(path: &Path) -> Result<Self> {
        trace!("Opening config from {path:?}");

        let mut f = fs::File::open(path).await?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await?;

        Ok(serde_json::from_slice(&buf)?)
    }

    pub async fn read_oidc_signing_key(&self) -> Result<String> {
        let absolute = self.oidc_signing_key.canonicalize()?;
        trace!("Reading OIDC signing key from {absolute:?}");
        Self::read_pem(&absolute).await
    }

    pub async fn read_oidc_public_key(&self) -> Result<String> {
        let absolute = self.oidc_public_key.canonicalize()?;
        trace!("Reading OIDC public key from {absolute:?}");
        Self::file_read_string(&absolute).await
    }

    async fn read_pem(p: &Path) -> Result<String> {
        let contents = Self::file_read_string(p).await?;
        let pem = pem::parse(contents.as_bytes())?;
        Ok(pem.to_string())
    }

    async fn file_read_string(p: &Path) -> Result<String> {
        let mut f = fs::File::open(p).await?;
        let mut buf = String::new();
        f.read_to_string(&mut buf).await?;
        Ok(buf)
    }
}

pub async fn get_config() -> Result<Config> {
    let env = EnvConfig::new()?;
    let config = Config::open(&env.config_path).await?;
    Ok(config)
}

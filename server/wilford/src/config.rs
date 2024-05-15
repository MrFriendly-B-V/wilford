use color_eyre::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncReadExt;

#[derive(Debug, Deserialize)]
struct EnvConfig {
    config_path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub espo: EspoConfig,
    pub default_client: DefaultClientConfig,
    pub oidc_signing_key: PathBuf,
    pub oidc_public_key: PathBuf,
    pub oidc_issuer: String,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub ui_login_path: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri_endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct EspoConfig {
    pub host: String,
    pub api_key: String,
    pub secret_key: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct DefaultClientConfig {
    pub redirect_uri: String,
}

impl EnvConfig {
    fn new() -> Result<Self> {
        Ok(envy::from_env()?)
    }
}

impl Config {
    async fn open(path: &Path) -> Result<Self> {
        let mut f = fs::File::open(path).await?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await?;

        Ok(serde_json::from_slice(&buf)?)
    }

    pub async fn read_oidc_signing_key(&self) -> Result<String> {
        Self::read_pem(&self.oidc_signing_key).await
    }

    pub async fn read_oidc_public_key(&self) -> Result<String> {
        Self::file_read_string(&self.oidc_public_key).await
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

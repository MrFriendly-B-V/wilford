use base64::Engine;
use espocrm_rs::{EspoApiClient, Method};
use reqwest::{Result, StatusCode};
use serde::Deserialize;
use tracing::warn;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EspoUser {
    pub id: String,
    pub name: String,
    pub email_address: String,
    pub is_active: bool,
    #[serde(rename(deserialize = "type"))]
    pub user_type: String,
}

pub enum LoginStatus {
    Ok(String),
    SecondStepRequired,
    Err,
}

impl EspoUser {
    pub async fn get_by_id(client: &EspoApiClient, id: &str) -> Result<Self> {
        Ok(client
            .request::<(), &str>(Method::Get, &format!("User/{id}"), None, None)
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn try_login(
        host: &str,
        username: &str,
        password: &str,
        totp_code: Option<&str>,
    ) -> Result<LoginStatus> {
        let username_password =
            base64::engine::general_purpose::STANDARD.encode(format!("{username}:{password}"));

        let mut request = reqwest::Client::new()
            .get(format!("{host}/api/v1/App/user"))
            .header("Authorization", format!("Basic {username_password}"))
            .header("Espo-Authorization", &username_password)
            .header("Espo-Authorization-By-Token", "false")
            .header("Espo-Authorization-Create-Token-Secret", "true");

        if let Some(totp) = totp_code {
            request = request.header("Espo-Authorization-Code", totp);
        }

        let result = request.send().await?;

        match result.status() {
            StatusCode::OK => {
                #[derive(Deserialize)]
                struct Response {
                    user: User,
                }

                #[derive(Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct User {
                    id: String,
                    is_active: bool,
                }

                let payload: Response = result.json().await?;
                if payload.user.is_active {
                    Ok(LoginStatus::Ok(payload.user.id))
                } else {
                    Ok(LoginStatus::Err)
                }
            }
            StatusCode::UNAUTHORIZED => {
                #[derive(Deserialize)]
                struct Response {
                    message: String,
                }

                let payload: Response = result.json().await?;
                if payload.message.eq("enterTotpCode") {
                    Ok(LoginStatus::SecondStepRequired)
                } else {
                    Ok(LoginStatus::Err)
                }
            }
            _ => {
                warn!("Espo status: {:?}", result.status());
                Ok(LoginStatus::Err)
            }
        }
    }
}

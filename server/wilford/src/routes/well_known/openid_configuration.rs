use crate::routes::appdata::WConfig;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct OpenidConfiguration {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    response_types_supported: Vec<String>,
    grant_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    jwks_uri: String,
}

pub async fn openid_configuration(config: WConfig) -> web::Json<OpenidConfiguration> {
    web::Json(OpenidConfiguration {
        issuer: config.oidc_issuer.clone(),
        authorization_endpoint: config.http.authorization_endpoint.clone(),
        token_endpoint: config.http.token_endpoint.clone(),
        response_types_supported: vec![
            "code".to_string(),
            "id_token token".to_string(),
            "token".to_string(),
        ],
        grant_types_supported: vec!["authorization_code".to_string(), "implicit".to_string()],
        id_token_signing_alg_values_supported: vec!["RS256".to_string()],
        jwks_uri: config.http.jwks_uri_endpoint.clone(),
    })
}

use crate::routes::appdata::WConfig;
use actix_web::web;
use serde::Serialize;

/// # Further reading
/// - [OpenID Connect Discovery 1.0, Section 4.1](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderConfig)
#[derive(Serialize)]
pub struct OpenidConfiguration {
    /// The isser of all JWT tokens
    issuer: String,
    /// The server's endpoint for authorization
    authorization_endpoint: String,
    /// The server's endpoint for exchanging a code or refresh token for an access token
    token_endpoint: String,
    /// The response types we support in the authorization endpoint
    response_types_supported: Vec<String>,
    /// The grant types we support in the token endpoint
    grant_types_supported: Vec<String>,
    /// The algorithms we support for signing JWTs
    id_token_signing_alg_values_supported: Vec<String>,
    /// The endpoint for where the JWKS document can found
    jwks_uri: String,
}

/// Get the OpenID configuration for this server
///
/// # Further reading
/// - [OpenID Connect Discovery 1.0, Section 4](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderConfig)
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

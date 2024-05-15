use crate::routes::appdata::WOidcPublicKey;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Jwks {
    keys: Vec<Key>,
}

#[derive(Serialize)]
pub struct Key {
    kty: String,
    r#use: String,
    alg: String,
    k: String,
}

pub async fn jwks(oidc_public_key: WOidcPublicKey) -> web::Json<Jwks> {
    web::Json(Jwks {
        keys: vec![Key {
            kty: "RSA".to_string(),
            r#use: "verify".to_string(),
            alg: "RS256".to_string(),
            k: (**oidc_public_key).0.clone(),
        }],
    })
}

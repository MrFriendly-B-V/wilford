use actix_web::web;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use rsa::{BigUint, RsaPublicKey};
use rsa::pkcs8::DecodePublicKey;
use rsa::traits::PublicKeyParts;
use serde::{Serialize, Serializer};

use crate::routes::appdata::WOidcPublicKey;
use crate::routes::error::WebResult;

#[derive(Serialize)]
pub struct Jwks {
    keys: Vec<Key>,
}

#[derive(Serialize)]
pub struct Key {
    kty: String,
    r#use: String,
    alg: String,
    kid: String,
    key_ops: Vec<String>,
    // k: String,
    #[serde(serialize_with = "serialize")]
    n: BigUint,
    #[serde(serialize_with = "serialize")]
    e: BigUint,
}

pub async fn jwks(oidc_public_key: WOidcPublicKey) -> WebResult<web::Json<Jwks>> {
    let public_key = RsaPublicKey::from_public_key_pem(&oidc_public_key.0)?;

    Ok(web::Json(Jwks {
        keys: vec![Key {
            kty: "RSA".to_string(),
            r#use: "sig".to_string(),
            alg: "RS256".to_string(),
            kid: "rsa".to_string(), // We dont have a kid
            key_ops: vec![
                "verify".to_string(),
            ],
            // k: (**oidc_public_key).0.clone(),
            n: public_key.n().clone(),
            e: public_key.e().clone()
        }],
    }))
}

pub fn serialize<S>(value: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = value.to_bytes_be();
    let base64 = BASE64_URL_SAFE_NO_PAD.encode(bytes.as_slice());
    serializer.serialize_str(&base64)
}
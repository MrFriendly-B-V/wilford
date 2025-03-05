use actix_web::web;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use rsa::pkcs8::DecodePublicKey;
use rsa::traits::PublicKeyParts;
use rsa::{BigUint, RsaPublicKey};
use serde::{Serialize, Serializer};

use crate::routes::appdata::WOidcPublicKey;
use crate::routes::error::WebResult;

/// The JWKS (JSON Web Key Set) document.
/// See also [RFC 7517, Section 5.1](https://datatracker.ietf.org/doc/html/rfc7517#section-5.1)
#[derive(Serialize)]
pub struct Jwks {
    /// The public keys we use
    keys: Vec<Jwk>,
}

/// A JWK, a JSON Web Key.
///
/// # Further reading
/// - General format: [RFC 7517, Section 4](https://datatracker.ietf.org/doc/html/rfc7517#section-4)
/// - RSA Public key extra's: [RFC 7518, Section 6.3.1](https://datatracker.ietf.org/doc/html/rfc7518#section-6.3.1)
#[derive(Serialize)]
pub struct Jwk {
    kty: String,
    r#use: String,
    alg: String,
    kid: String,
    key_ops: Vec<String>,
    // k: String,
    #[serde(serialize_with = "serialize_bigint")]
    n: BigUint,
    #[serde(serialize_with = "serialize_bigint")]
    e: BigUint,
}

/// Endpoint for fetching the JWKS document.
/// This endpoint is used by OAuth clients to get our public keys,
/// so they can validate that the ID tokens are issued by us.
///
/// # Further reading
/// - [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517)
pub async fn jwks(oidc_public_key: WOidcPublicKey) -> WebResult<web::Json<Jwks>> {
    let public_key = RsaPublicKey::from_public_key_pem(&oidc_public_key.0)?;

    Ok(web::Json(Jwks {
        keys: vec![Jwk {
            kty: "RSA".to_string(),
            r#use: "sig".to_string(),
            alg: "RS256".to_string(),
            kid: "rsa".to_string(), // We don't have a kid
            key_ops: vec!["verify".to_string()],
            n: public_key.n().clone(),
            e: public_key.e().clone(),
        }],
    }))
}

/// Base64 BigInt serializer.
/// Uses base64 in the format URL safe, without padding.
///
/// # Further reading
/// - [RFC 7518, Section 6.3.1.1](https://datatracker.ietf.org/doc/html/rfc7518#section-6.3.1.1)
pub fn serialize_bigint<S>(value: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = value.to_bytes_be();
    let base64 = BASE64_URL_SAFE_NO_PAD.encode(bytes.as_slice());
    serializer.serialize_str(&base64)
}

use crate::config::Config;
use actix_web::web;
use database::driver::Database;

pub type WDatabase = web::Data<Database>;
pub type WConfig = web::Data<Config>;

pub type WOidcSigningKey = web::Data<OidcSigningKey>;
pub type WOidcPublicKey = web::Data<OidcPublicKey>;

pub struct OidcSigningKey(pub String);
pub struct OidcPublicKey(pub String);

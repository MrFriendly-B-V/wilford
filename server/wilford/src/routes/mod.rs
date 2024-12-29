use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod appdata;
mod auth;
mod error;
mod oauth;
mod v1;
mod well_known;

use crate::authorization::combined::CombinedAuthorizationProviderError;
use crate::authorization::espo::EspoAuthorizationProviderError;
use crate::authorization::local_provider::LocalAuthorizationProviderError;
use crate::authorization::AuthorizationError;
use crate::routes::error::{WebError, WebErrorKind};
pub use appdata::*;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.configure(well_known::Router::configure).service(
            web::scope("/api")
                .configure(v1::Router::configure)
                .configure(oauth::Router::configure),
        );
    }
}

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    fn unwrap_left(self) -> A {
        match self {
            Self::Left(a) => a,
            Self::Right(_) => panic!("Unwrapped on a Right value"),
        }
    }
}

/// Convert a result with an authorization error to a result with a web error.
/// The `Ok` value is returned in the `Left` branch. The `Right` branch
/// returns an empty tuple if the error is `TotpRequired`.
fn auth_error_to_web_error<T>(
    e: Result<T, AuthorizationError<CombinedAuthorizationProviderError>>,
) -> Result<Either<T, ()>, WebError> {
    match e {
        Ok(t) => Ok(Either::Left(t)),
        Err(AuthorizationError::TotpNeeded) => Ok(Either::Right(())),
        Err(AuthorizationError::InvalidCredentials) => Err(WebErrorKind::Unauthorized.into()),
        Err(AuthorizationError::UnsupportedOperation) => Err(WebErrorKind::Unsupported.into()),
        Err(AuthorizationError::AlreadyExists) => Err(WebErrorKind::BadRequest.into()),
        Err(AuthorizationError::Other(e)) => Err(match e {
            CombinedAuthorizationProviderError::Local(e) => match e {
                LocalAuthorizationProviderError::Database(e) => e.into(),
                LocalAuthorizationProviderError::Hashing(_) => {
                    WebErrorKind::InternalServerError.into()
                }
            },
            CombinedAuthorizationProviderError::EspoCrm(e) => match e {
                EspoAuthorizationProviderError::Database(e) => e.into(),
                EspoAuthorizationProviderError::Espocrm(_) => {
                    WebErrorKind::InternalServerError.into()
                }
            },
        }),
    }
}

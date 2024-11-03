use actix_web::http::StatusCode;
use actix_web::ResponseError;
use std::fmt;
use std::fmt::{Formatter, Write};
use thiserror::Error;
use tracing_error::SpanTrace;

pub type WebResult<T> = Result<T, WebError>;

#[derive(Error)]
#[error("{}", kind)]
pub struct WebError {
    kind: WebErrorKind,
    context: SpanTrace,
}

impl fmt::Debug for WebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)?;
        f.write_char('\n')?;
        f.write_str(&self.context.to_string())?;
        f.write_char('\n')?;
        Ok(())
    }
}

impl<E> From<E> for WebError
where
    E: Into<WebErrorKind>,
{
    #[track_caller]
    fn from(value: E) -> Self {
        Self {
            kind: value.into(),
            context: SpanTrace::capture(),
        }
    }
}

impl WebError {
    pub fn new(kind: WebErrorKind) -> Self {
        Self {
            kind,
            context: SpanTrace::capture(),
        }
    }
}

#[derive(Debug, Error)]
pub enum WebErrorKind {
    #[error("Not found")]
    NotFound,
    #[error("Bad request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid internal state")]
    InvalidInternalState,
    #[error("Forbidden")]
    Forbidden,
    #[error("{0}")]
    Database(#[from] database::driver::Error),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Failed to parse PKCS8 SPKI: {0}")]
    RsaPkcs8Spki(#[from] rsa::pkcs8::spki::Error),
    #[error("Authorization error: {0}")]
    AuthorizationProvider(#[from] crate::authorization_backends::AuthorizationProviderError),
}

impl ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            WebErrorKind::NotFound => StatusCode::NOT_FOUND,
            WebErrorKind::BadRequest => StatusCode::BAD_REQUEST,
            WebErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            WebErrorKind::Forbidden => StatusCode::FORBIDDEN,
            WebErrorKind::InvalidInternalState => StatusCode::INTERNAL_SERVER_ERROR,
            WebErrorKind::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebErrorKind::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            WebErrorKind::RsaPkcs8Spki(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebErrorKind::AuthorizationProvider(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

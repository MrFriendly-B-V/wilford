use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::HeaderValue;
use reqwest::header::HeaderName;

pub struct Uncached<T>(T);

impl<T: Responder> Uncached<T> {
    pub fn new(responder: T) -> Self {
        Self(responder)
    }
}

impl<T: Responder> Responder for Uncached<T> {
    type Body = T::Body;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut response = self.0.respond_to(req);
        let headers = response.headers_mut();
        headers.insert(HeaderName::from_static("cache-control"), HeaderValue::from_static("no-store"));

        response
    }
}
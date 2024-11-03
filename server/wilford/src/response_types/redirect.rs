use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub struct Redirect(pub String);

impl Redirect {
    pub fn new(location: String) -> Self {
        Self(location)
    }
}

impl Responder for Redirect {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(StatusCode::FOUND)
            .insert_header(("Location", self.0))
            .finish()
    }
}

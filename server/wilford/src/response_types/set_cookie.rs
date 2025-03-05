use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::cookie::{Cookie, Expiration, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder};
use std::borrow::Cow;

/// Set a cookie on response
pub struct SetCookie<'c, I> {
    /// The cookie name
    name: Cow<'c, str>,
    /// The cookie value
    value: Cow<'c, str>,
    /// The inner responder
    responder: I,
}

impl<'c, I> SetCookie<'c, I> {
    /// Create a new cookie and responds with the responder provided.
    pub fn new<N, V>(name: N, value: V, responder: I) -> Self
    where
        I: Responder,
        N: Into<Cow<'c, str>>,
        V: Into<Cow<'c, str>>,
    {
        Self {
            name: name.into(),
            value: value.into(),
            responder,
        }
    }
}

impl<I> Responder for SetCookie<'_, I>
where
    I: Responder,
{
    type Body = I::Body;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Could be a JSON body e.g.
        let mut inner_response = self.responder.respond_to(req);
        let mut cookie = Cookie::new(self.name, self.value);
        cookie.set_secure(true);

        // Perhaps make this configurable?
        cookie.set_expires(Expiration::DateTime(
            OffsetDateTime::now_utc() + Duration::days(30),
        ));

        cookie.set_same_site(SameSite::None);
        cookie.set_path("/");

        inner_response.add_cookie(&cookie).unwrap();
        inner_response
    }
}

/// Set a cookie, but not always. Useful for when a responder should not always
/// set a cookie.
pub struct MaybeCookie<'c, I> {
    /// The cookie to set
    cookie: Option<SetCookie<'c, I>>,
    /// If the cookie is not set, the responder to use
    responder: Option<I>,
}

impl<'c, I> MaybeCookie<'c, I> {
    /// Respond with a cookie
    pub fn some(cookie: SetCookie<'c, I>) -> Self {
        Self {
            cookie: Some(cookie),
            responder: None,
        }
    }
}

impl<I> MaybeCookie<'_, I>
where
    I: Responder,
{
    /// Do not respond with a cookie, but rather with the responder provided.
    pub fn none(responder: I) -> Self {
        Self {
            cookie: None,
            responder: Some(responder),
        }
    }
}

impl<I> Responder for MaybeCookie<'_, I>
where
    I: Responder,
{
    type Body = I::Body;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match (self.cookie, self.responder) {
            // Send the cookie, and let the cookie handle the inner responder
            (Some(c), _) => c.respond_to(req),
            // Don't send a cookie, let us handle the inner cookie
            (None, Some(i)) => i.respond_to(req),
            _ => unreachable!(),
        }
    }
}

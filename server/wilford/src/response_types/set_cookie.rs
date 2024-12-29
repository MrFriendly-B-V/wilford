use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::cookie::{Cookie, Expiration, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder};
use std::borrow::Cow;

pub struct SetCookie<'c, I> {
    k: Cow<'c, str>,
    v: Cow<'c, str>,
    i: I,
}

impl<'c, I> SetCookie<'c, I> {
    pub fn new<N, V>(k: N, v: V, i: I) -> Self
    where
        I: Responder,
        N: Into<Cow<'c, str>>,
        V: Into<Cow<'c, str>>,
    {
        Self {
            k: k.into(),
            v: v.into(),
            i,
        }
    }
}

impl<'c, I> Responder for SetCookie<'c, I>
where
    I: Responder,
{
    type Body = I::Body;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut inner_response = self.i.respond_to(req);
        let mut cookie = Cookie::new(self.k, self.v);
        cookie.set_secure(true);
        cookie.set_expires(Expiration::DateTime(
            OffsetDateTime::now_utc() + Duration::days(30),
        ));
        cookie.set_same_site(SameSite::None);
        cookie.set_path("/");

        inner_response.add_cookie(&cookie).unwrap();
        inner_response
    }
}

pub struct MaybeCookie<'c, I> {
    cookie: Option<SetCookie<'c, I>>,
    i: Option<I>,
}

impl<'c, I> MaybeCookie<'c, I> {
    pub fn some(cookie: SetCookie<'c, I>) -> Self {
        Self {
            cookie: Some(cookie),
            i: None,
        }
    }
}

impl<'c, I> MaybeCookie<'c, I>
where
    I: Responder,
{
    pub fn none(i: I) -> Self {
        Self {
            cookie: None,
            i: Some(i),
        }
    }
}

impl<'c, I> Responder for MaybeCookie<'c, I>
where
    I: Responder,
{
    type Body = I::Body;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match (self.cookie, self.i) {
            (Some(c), _) => c.respond_to(req),
            (None, Some(i)) => i.respond_to(req),
            _ => unreachable!(),
        }
    }
}

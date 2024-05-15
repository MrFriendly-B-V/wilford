use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod appdata;
mod auth;
mod error;
mod oauth;
mod v1;
mod well_known;

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

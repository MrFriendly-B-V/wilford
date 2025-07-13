mod jwks;
mod openid_configuration;

use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/.well-known")
                // Route defined in
                // OpenID Connect Discovery 1.0, Section 4, https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderConfig
                .route(
                    "/openid-configuration",
                    web::get().to(openid_configuration::openid_configuration),
                )
                .route("/jwks.json", web::get().to(jwks::jwks)),
        );
    }
}

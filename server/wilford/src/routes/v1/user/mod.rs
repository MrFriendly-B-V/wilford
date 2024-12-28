use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod change_password;
mod info;
mod list;
mod permitted_scopes;
mod register;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/user")
                .configure(permitted_scopes::Router::configure)
                .route("/info", web::get().to(info::info))
                .route("/list", web::get().to(list::list))
                .route(
                    "/change-password",
                    web::post().to(change_password::change_password),
                )
                .route("/register", web::post().to(register::register)),
        );
    }
}

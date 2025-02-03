use crate::config::Config;
use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;
use database::user::UserEmailVerification;

mod change_email;
mod change_password;
mod info;
mod list;
mod password_forgotten;
mod permitted_scopes;
mod register;
mod registration_required;
mod supports_password_change;
mod verify_email;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/user")
                .configure(permitted_scopes::Router::configure)
                .route("/info", web::get().to(info::info))
                .route("/list", web::get().to(list::list))
                .route(
                    "/registration-required",
                    web::get().to(registration_required::registration_required),
                )
                .route(
                    "password-forgotten",
                    web::post().to(password_forgotten::password_forgotten),
                )
                .route(
                    "/change-password",
                    web::post().to(change_password::change_password),
                )
                .route("/change-email", web::post().to(change_email::change_email))
                .route(
                    "/supports-password-change",
                    web::get().to(supports_password_change::supports_password_change),
                )
                .route("/register", web::post().to(register::register))
                .route("/verify-email", web::post().to(verify_email::verify_email)),
        );
    }
}

fn email_verify_link(config: &Config, verification: &UserEmailVerification) -> String {
    format!(
        "{}?code={}&user_id={}",
        &config.http.ui_email_verification_path,
        &verification.verification_code,
        &verification.user_id
    )
}

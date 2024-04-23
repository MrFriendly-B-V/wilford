use crate::config::{get_config, DefaultClientConfig};
use actix_cors::Cors;
use actix_route_config::Routable;
use actix_web::{web, App, HttpServer};
use color_eyre::Result;
use database::driver::Database;
use database::oauth2_client::OAuth2Client;
use espocrm_rs::EspoApiClient;
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod config;
mod espo;
mod routes;
mod response_types;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();

    let config = get_config().await?;
    let database = Database::new(
        &config.database.user,
        &config.database.password,
        &config.database.host,
        &config.database.database,
    )
    .await?;
    let espo_client = EspoApiClient::new(&config.espo.host)
        .set_api_key(&config.espo.api_key)
        .set_secret_key(&config.espo.secret_key)
        .build();

    ensure_internal_oauth_client_exists(&database, &config.default_client).await?;

    let w_database = web::Data::new(database);
    let w_config = web::Data::new(config);
    let w_espo = web::Data::new(espo_client);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::<NoiselessRootSpanBuilder>::new())
            .app_data(w_database.clone())
            .app_data(w_config.clone())
            .app_data(w_espo.clone())
            .configure(routes::Router::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

async fn ensure_internal_oauth_client_exists(
    driver: &Database,
    config: &DefaultClientConfig,
) -> Result<()> {
    let clients = OAuth2Client::list(driver).await?;
    if !clients.is_empty() {
        return Ok(());
    }

    let client = OAuth2Client::new(
        driver,
        "Wilford".to_string(),
        config.redirect_uri.clone(),
        true,
    )
    .await?;

    info!("No internal OAuth2 exists yet. Created a new one. This client is for logging in with Wilford itself.");
    info!("Default client `client_id`: {}", client.client_id);
    info!("Default client `client_secret`: {}", client.client_secret);

    Ok(())
}

fn install_tracing() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
    }

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(layer().compact())
        .init();
}

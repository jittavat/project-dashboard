#![allow(dependency_on_unit_never_type_fallback)]

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();

    let cfg = config::Settings::from_env().expect("Failed to load config");

    let pool = db::connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let host = cfg.server_host.clone();
    let port = cfg.server_port;

    tracing::info!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .configure(routes::configure)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

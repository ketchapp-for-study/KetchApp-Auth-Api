mod config;
mod handlers;
mod models;

use crate::config::openapi::ApiDoc;
use crate::handlers::route_config;
use actix_cors::Cors;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use common::config::app_config::AppConfig;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOCATOR: dhat::DhatAlloc<std::alloc::System> = dhat::DhatAlloc(std::alloc::System);

type DbPool = Pool<ConnectionManager<PgConnection>>;

const SERVER_HOST: &str = "127.0.0.1";

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    #[cfg(feature = "dhat-heap")]
    let _dhat = dhat::Dhat::start_heap_profiling();

    dotenvy::dotenv().ok();
    let app_config = AppConfig::from_files("./config.toml").expect("Failed to load AppConfig");
    let log_level = app_config
        .rust_log
        .clone()
        .unwrap_or_else(|| "debug".to_string());
    tracing_subscriber::fmt()
        .with_max_level(log_level.parse().unwrap_or(tracing::Level::DEBUG))
        .init();

    info!("Starting server...");

    // Use AppConfig struct and loader from common
    let port = app_config.port;
    let database_url = app_config.database_url.clone();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    info!("Connected to database successfully");

    // println!("Loaded settings: {:?}", settings);
    // let port: u16 = settings.get_int("port").unwrap_or(8080) as u16;
    let server_address = format!("{}:{}", SERVER_HOST, port);

    info!("Starting HTTP server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .wrap(Logger::default())
            .wrap(Cors::default().allowed_header(CONTENT_TYPE))
            .configure(route_config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((SERVER_HOST, port))?
    .run()
    .await
}

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use ketchapp_auth_api::config::app_config::AppConfig;
use ketchapp_auth_api::config::open_api::ApiDoc;
use ketchapp_auth_api::handlers::route_config;
use std::env;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    let app_config = AppConfig::from_files().expect("Failed to load AppConfig");

    // Safely set the environment variable without using unsafe
    let rust_log = app_config
        .rust_log
        .clone()
        .unwrap_or_else(|| "info".to_string());
    env::set_var("RUST_LOG", &rust_log);

    tracing_subscriber::fmt::init();

    info!("Starting server...");

    let host = app_config.host.clone();
    let port = app_config.port;
    let database_url = app_config.database_url.clone();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let server_address = format!("{}:{}", host, port);

    info!("Starting HTTP server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials(),
            )
            .configure(route_config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((host, port))?
    .run()
    .await
}

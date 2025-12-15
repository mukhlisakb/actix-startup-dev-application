use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use infrastructure::database::Database;
use log::info;
use shared::config::DatabaseConfig;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting server...");

    let db_config = DatabaseConfig::from_env();
    info!(
        "Connecting to database at {}:{}",
        db_config.host, db_config.port
    );

    let db = match Database::new_connection(&db_config).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db_data.clone())
            .service(routes::health::health_check)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

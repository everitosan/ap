use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use tracing::info;
use tracing_actix_web::TracingLogger;

use ap_back::config::{create_pool, Settings};
use ap_back::modules::auth::infrastructure::WhatsAppPhoneNotifier;
// use ap_back::modules::auth::infrastructure::StubPhoneNotifier;
use ap_back::server::{configure_routes, shutdown_signal};
use ap_back::shared::{AppState, CookieEncryptor};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,sqlx=warn")
        .init();

    info!("Starting AP Backend Server...");

    // Load configuration
    dotenvy::dotenv().ok();
    let settings = Settings::from_env().expect("Failed to load settings");

    // Create database connection pool
    let db_pool = create_pool(&settings.database)
        .await
        .expect("Failed to create database pool");

    info!("Database pool created successfully");

    // Create WhatsApp phone notifier
    let phone_notifier = Arc::new(WhatsAppPhoneNotifier::new(
        settings.whatsapp.clone(),
        &settings.whatsapp_otp_template,
    ));

    // let phone_notifier =  Arc::new(StubPhoneNotifier);

    // Create cookie encryptor
    let cookie_encryptor = CookieEncryptor::new(&settings.server.cookie_secret);

    // Create application state
    let app_state = web::Data::new(AppState::new(db_pool.clone(), phone_notifier, cookie_encryptor));

    let bind_address = format!("{}:{}", settings.server.host, settings.server.port);
    let frontend_url = settings.server.frontend_url.clone();
    info!("Binding to {}", bind_address);
    info!("CORS allowed origin: {}", frontend_url);

    // Create HTTP server
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(TracingLogger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(configure_routes)
    })
    .bind(&bind_address)?
    .shutdown_timeout(30)
    .run();

    // Get server handle for graceful shutdown
    let server_handle = server.handle();

    // Spawn shutdown signal handler
    tokio::spawn(async move {
        shutdown_signal().await;
        info!("Shutting down HTTP server...");
        server_handle.stop(true).await;
    });

    // Run server
    server.await?;

    // Cleanup: close database pool
    info!("Closing database connections...");
    db_pool.close().await;

    info!("Server shutdown complete");
    Ok(())
}

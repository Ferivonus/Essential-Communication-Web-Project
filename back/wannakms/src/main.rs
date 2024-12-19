use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

mod handlers;
mod schemas;
use handlers::{form_handlers, message_handlers, wailing_wall_handlers};

#[allow(dead_code)]
pub struct AppState {
    db: MySqlPool,
}

// Function to initialize logging
fn init_logging() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
}

// Function to initialize the database pool
async fn init_db_pool() -> MySqlPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file
    init_logging(); // Initialize the logger

    // Set up database pool
    let db_pool = init_db_pool().await;
    println!("✅ Connection to the database is successful!");

    println!(
        "{}",
        std::env::var("APPLICATION_USERNAME").expect("APPLICATION_USERNAME must be set")
    );
    // Fetch connection links from the database
    let connection_links = fetch_connection_links(&db_pool).await.unwrap();

    // Configure and start the HTTP server
    HttpServer::new(move || {
        // Initialize CORS and dynamically add origins from the database
        let mut cors = Cors::default()
            .allowed_origin("http://127.0.0.1:1420")
            .allowed_origin("http://127.0.0.1:4875")
            .allowed_origin("http://127.0.0.1:4173")
            .allowed_origin("http://wulvahiv5zqfrgqo3ceavahsfakxjvp2oqe53sihlq2cuqhzqmea6yqd.onion")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        // Dynamically add connection links to CORS
        for link in &connection_links {
            cors = cors.allowed_origin(link);
        }

        App::new()
            .app_data(web::Data::new(AppState {
                db: db_pool.clone(),
            }))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .configure(message_handlers::message_handler_config)
            .configure(form_handlers::form_handler_config)
            .configure(wailing_wall_handlers::message_handler_config)
    })
    .bind(("127.0.0.1", 3000))? // 4875 old port
    .run()
    .await
}

// Fetch connection links from both user_wired_connection_keys and other_wired_connection_keys
async fn fetch_connection_links(db_pool: &MySqlPool) -> Result<Vec<String>, sqlx::Error> {
    let mut links = Vec::new();

    // Query to fetch links from user_wired_connection_keys
    let user_links = sqlx::query!("SELECT connection_link FROM user_wired_connection_keys")
        .fetch_all(db_pool)
        .await?
        .into_iter()
        .map(|record| record.connection_link)
        .collect::<Vec<String>>();

    // Query to fetch links from other_wired_connection_keys
    let other_links = sqlx::query!("SELECT connection_link FROM other_wired_connection_keys")
        .fetch_all(db_pool)
        .await?
        .into_iter()
        .map(|record| record.connection_link)
        .collect::<Vec<String>>();

    // Combine both user and other server links into a single vector
    links.extend(user_links);
    links.extend(other_links);

    Ok(links)
}

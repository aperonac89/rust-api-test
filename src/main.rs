mod handlers;
mod models;
mod routes;

use crate::routes::{health, notes};
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("error: not possible to load db url");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(30)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("connection to the DB completed successfully");
            pool
        }
        Err(e) => {
            let msg = format!("error: couldnt connect to the db: {}", e);
            println!("{}", msg);
            std::process::exit(1);
        }
    };

    // Staring the server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(notes::config)
            .configure(health::config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

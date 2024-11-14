use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use track_my_stuff_api::database;
use track_my_stuff_api::models::items::routes::item_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match database::connection::establish_connection(&database_url) {
        Ok(_connection) => {
            println!("Connected to the database!");
            println!("Database URL: {}", database_url);

            let manager = ConnectionManager::<PgConnection>::new(database_url);
            let pool = Pool::builder()
                .build(manager)
                .expect("Failed to create pool.");

            match HttpServer::new(move || {
                App::new()
                    .wrap(Logger::default())
                    .app_data(web::Data::new(pool.clone()))
                    .service(web::scope("/api").configure(item_routes))
            })
            .bind(("127.0.0.1", 8080))
            {
                Ok(server) => {
                    println!("Server started successfully");
                    println!("API Running on: http://127.0.0.1:8080/api/");
                    server.run().await
                }
                Err(e) => {
                    eprintln!("Error starting server: {}", e);
                    Err(e)
                }
            }
        }
        Err(error) => {
            eprintln!("Error connecting to the database: {}", error);
            std::process::exit(1);
        }
    }
}

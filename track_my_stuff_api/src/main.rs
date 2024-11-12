use dotenv::dotenv;
mod database;

fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match database::establish_connection(&database_url) {
        Ok(_connection) => {
            // Use the connection
            println!("Connected to the database!");
        }
        Err(error) => {
            eprintln!("Error connecting to the database: {}", error);
        }
    }
}

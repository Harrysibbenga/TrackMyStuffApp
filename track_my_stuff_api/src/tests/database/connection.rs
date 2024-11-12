#[cfg(test)]
mod connection_tests {
    use crate::database;
    use dotenv::dotenv;
    use std::env;

    fn setup_env() {
        dotenv().ok();
    }

    #[test]
    fn test_database_connection_success() {
        setup_env();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        match database::connection::establish_connection(&database_url) {
            Ok(_connection) => {
                // Connection should be successful
                assert!(true);
            }
            Err(_error) => {
                // Connection should not fail
                assert!(false, "Expected successful connection, but got an error");
            }
        }
    }

    #[test]
    fn test_database_connection_failure() {
        setup_env();

        env::set_var("TEST_URL", "invalid_database_url");

        match database::connection::establish_connection(&env::var("TEST_URL").unwrap()) {
            Ok(_connection) => {
                // Connection should fail
                assert!(
                    false,
                    "Expected connection failure, but got a successful connection"
                );
            }
            Err(_error) => {
                // Connection should fail
                assert!(true);
            }
        }
    }
}

#[cfg(test)]
mod item_model_tests {
    use std::env;

    // crates
    use crate::database;
    use crate::models;

    use chrono::NaiveDateTime;
    use database::connection::establish_connection;
    use diesel::PgConnection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use dotenv::dotenv;
    use models::items::crud::{create_item, delete_item, get_item_by_id, get_items, update_item};
    use models::items::models::{CreateItem, Item, UpdateItem};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    fn setup_env() {
        dotenv().ok();
    }

    fn run_migrations(connection: &mut PgConnection) {
        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
    }

    fn establish_test_connection() -> PgConnection {
        setup_env();
        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let mut connection =
            establish_connection(&database_url).expect("Failed to establish connection");
        run_migrations(&mut connection);
        connection
    }

    #[test]
    #[allow(deprecated)]
    fn test_create_item() {
        let connection = &mut establish_test_connection();
        let name: String = "Test Item".to_string();
        let description: String = "This is a test item".to_string();
        let expected_arrival_date: NaiveDateTime = NaiveDateTime::from_timestamp(1_632_112_000, 0);

        let new_item: CreateItem = CreateItem {
            name: name.clone(),
            description: Some(description.clone()),
            expected_arrival_date: Some(expected_arrival_date),
            item_received: Some(false),
        };

        let item: Item = create_item(connection, new_item).unwrap();

        assert_eq!(item.name, name);
        assert_eq!(item.description, Some(String::from(description)));
        assert_eq!(item.expected_arrival_date, expected_arrival_date);
    }

    #[test]
    fn test_get_items() {
        let connection = &mut establish_test_connection();
        let items = get_items(connection).unwrap();
        assert!(items.len() > 0);
    }

    #[test]
    fn test_get_item_by_id() {
        let connection = &mut establish_test_connection();

        let new_item = CreateItem {
            name: "New Test Item".to_string(),
            description: Some("This is a new test item".to_string()),
            expected_arrival_date: Some(NaiveDateTime::from_timestamp(1_632_112_000, 0)),
            item_received: Some(false),
        };

        let new_item: Item = create_item(connection, new_item).unwrap();

        let item = get_item_by_id(connection, new_item.id).unwrap();

        assert_eq!(item.id, new_item.id);
    }

    #[test]
    #[allow(deprecated)]
    fn test_update_item() {
        let connection = &mut establish_test_connection();
        let updated_item = UpdateItem {
            name: Some("Updated Test Item".to_string()),
            description: Some("This is an updated test item".to_string()),
            expected_arrival_date: Some(NaiveDateTime::from_timestamp(1_632_112_000, 0)),
            item_received: Some(true),
        };

        let new_item = CreateItem {
            name: "New Test Item".to_string(),
            description: Some("This is a new test item".to_string()),
            expected_arrival_date: Some(NaiveDateTime::from_timestamp(1_632_112_000, 0)),
            item_received: Some(false),
        };

        let new_item: Item = create_item(connection, new_item).unwrap();

        let item = update_item(connection, new_item.id, &updated_item).unwrap();

        assert_eq!(item.name, "Updated Test Item");
        assert_eq!(
            item.description,
            Some("This is an updated test item".to_string())
        );
        assert_eq!(
            item.expected_arrival_date,
            NaiveDateTime::from_timestamp(1_632_112_000, 0)
        );
        assert_eq!(item.item_received, Some(true).is_some());
    }

    #[test]
    #[allow(deprecated)]
    fn test_delete_item() {
        let connection = &mut establish_test_connection();

        let new_item = CreateItem {
            name: "New Test Item".to_string(),
            description: Some("This is a new test item".to_string()),
            expected_arrival_date: Some(NaiveDateTime::from_timestamp(1_632_112_000, 0)),
            item_received: Some(false),
        };

        let new_item: Item = create_item(connection, new_item).unwrap();

        delete_item(connection, new_item.id).unwrap();
        let result = get_item_by_id(connection, new_item.id);
        assert!(result.is_err());
    }

    #[test]
    #[allow(deprecated)]
    fn test_item_creation() {
        let item = Item {
            id: 1,
            name: String::from("Test Item"),
            description: Some(String::from("This is a test item")),
            expected_arrival_date: NaiveDateTime::from_timestamp(1_632_112_000, 0),
            item_received: false,
        };

        assert_eq!(item.id, 1);
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.description, Some(String::from("This is a test item")));
        assert_eq!(
            item.expected_arrival_date,
            NaiveDateTime::from_timestamp(1_632_112_000, 0)
        );
        assert_eq!(item.item_received, false);
    }

    #[test]
    #[allow(deprecated)]
    fn test_create_item_creation() {
        let new_item = CreateItem {
            name: "New Test Item".to_string(),
            description: Some("This is a new test item".to_string()),
            expected_arrival_date: Some(NaiveDateTime::from_timestamp(1_632_112_000, 0)),
            item_received: Some(false),
        };

        assert_eq!(new_item.name, "New Test Item");
        assert_eq!(
            new_item.description,
            Some("This is a new test item".to_string())
        );
        assert_eq!(
            new_item.expected_arrival_date,
            Some(NaiveDateTime::from_timestamp(1_632_112_000, 0))
        );
        assert_eq!(new_item.item_received, Some(false));
    }
}

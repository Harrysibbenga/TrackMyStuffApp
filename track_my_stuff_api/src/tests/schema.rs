#[cfg(test)]
mod tests {
    use super::*;
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;

    #[test]
    fn test_schema_definition() {
        let connection = SqliteConnection::establish(":memory:").unwrap();
        let result = diesel::sql_query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='items';",
        )
        .execute(&connection);

        assert!(result.is_ok());
    }
}

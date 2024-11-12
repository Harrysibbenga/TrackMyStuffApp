#[cfg(test)]
mod item_model_tests {
    use crate::models;
    use chrono::NaiveDateTime;
    use models::items::models::{Item, NewItem, UpdateItem};

    #[test]
    #[allow(deprecated)]
    fn test_update_item() {
        let update_item = UpdateItem {
            name: Some(String::from("Updated Test Item")),
            description: Some(String::from("This is an updated test item")),
            expected_arrival_date: Some(NaiveDateTime::from_timestamp(1_632_112_000, 0)),
            received: Some(true),
        };

        assert_eq!(update_item.name, Some(String::from("Updated Test Item")));
        assert_eq!(
            update_item.description,
            Some(String::from("This is an updated test item"))
        );
        assert_eq!(
            update_item.expected_arrival_date,
            Some(NaiveDateTime::from_timestamp(1_632_112_000, 0))
        );
        assert_eq!(update_item.received, Some(true));
    }

    #[test]
    #[allow(deprecated)]
    fn test_item_creation() {
        let item = Item {
            id: 1,
            name: String::from("Test Item"),
            description: Some(String::from("This is a test item")),
            expected_arrival_date: NaiveDateTime::from_timestamp(1_632_112_000, 0),
            received: false,
        };

        assert_eq!(item.id, 1);
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.description, Some(String::from("This is a test item")));
        assert_eq!(
            item.expected_arrival_date,
            NaiveDateTime::from_timestamp(1_632_112_000, 0)
        );
        assert_eq!(item.received, false);
    }

    #[test]
    #[allow(deprecated)]
    fn test_new_item_creation() {
        let new_item = NewItem {
            name: "New Test Item",
            description: Some("This is a new test item"),
            expected_arrival_date: NaiveDateTime::from_timestamp(1_632_112_000, 0),
        };

        assert_eq!(new_item.name, "New Test Item");
        assert_eq!(new_item.description, Some("This is a new test item"));
        assert_eq!(
            new_item.expected_arrival_date,
            NaiveDateTime::from_timestamp(1_632_112_000, 0)
        );
    }
}

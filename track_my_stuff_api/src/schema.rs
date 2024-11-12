// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        expected_arrival_date -> Timestamp,
        item_received -> Bool,
    }
}

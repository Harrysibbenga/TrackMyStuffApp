diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        expected_arrival_date -> Timestamp,
        received -> Bool,
    }
}

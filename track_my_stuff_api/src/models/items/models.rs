use super::schema::items;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub expected_arrival_date: NaiveDateTime,
    pub item_received: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemData {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = items)]
pub struct CreateItem {
    pub name: String,
    pub description: Option<String>,
    pub expected_arrival_date: Option<NaiveDateTime>,
    pub item_received: Option<bool>,
}

#[derive(AsChangeset, Debug, Deserialize, Serialize)]
#[diesel(table_name = items)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub description: Option<String>,
    pub expected_arrival_date: Option<NaiveDateTime>,
    pub item_received: Option<bool>,
}

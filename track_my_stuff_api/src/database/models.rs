use super::schema::items;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub expected_arrival_date: NaiveDateTime,
    pub received: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub expected_arrival_date: NaiveDateTime,
}

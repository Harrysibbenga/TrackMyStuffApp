use diesel::prelude::*;
use diesel::result::Error;

use super::models::{CreateItem, Item, UpdateItem};
use super::schema::items;

pub fn create_item(
    connection: &mut PgConnection,
    item_data: CreateItem,
) -> Result<Item, diesel::result::Error> {
    let new_item = CreateItem {
        name: item_data.name,
        description: item_data.description,
        expected_arrival_date: item_data.expected_arrival_date,
        item_received: item_data.item_received,
    };

    diesel::insert_into(items::table)
        .values(&new_item)
        .get_result(connection)
}

pub fn get_items(connection: &mut PgConnection) -> Result<Vec<Item>, Error> {
    items::table.load::<Item>(connection)
}

pub fn get_item_by_id(connection: &mut PgConnection, item_id: i32) -> Result<Item, Error> {
    items::table.find(item_id).first(connection)
}

pub fn update_item(
    connection: &mut PgConnection,
    item_id: i32,
    updated_item: &UpdateItem,
) -> Result<Item, Error> {
    diesel::update(items::table.find(item_id))
        .set(updated_item)
        .get_result(connection)
}

pub fn delete_item(connection: &mut PgConnection, item_id: i32) -> Result<(), Error> {
    diesel::delete(items::table.find(item_id)).execute(connection)?;
    Ok(())
}

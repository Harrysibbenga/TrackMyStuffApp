use super::crud::{create_item, delete_item, get_item_by_id, get_items, update_item};
use super::models::{CreateItem, UpdateItem};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::io::Write;

pub fn item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_item_handler)
        .service(get_item_handler)
        .service(get_item_by_id_handler)
        .service(delete_item_handler)
        .service(update_item_handler)
        .service(test_handler);
}

#[get("/test")]
async fn test_handler() -> impl Responder {
    println!("Test route accessed!");
    HttpResponse::Ok().body("Test route")
}

#[post("/items")]
async fn create_item_handler(
    item_data: web::Json<CreateItem>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error getting database connection");

    match create_item(&mut conn, item_data.into_inner()) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => {
            eprintln!("Failed to create item: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to create item: {}", e))
        }
    }
}

#[get("/items")]
async fn get_item_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error getting database connection");

    match get_items(&mut conn) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => {
            eprintln!("Failed to get item: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to get items: {}", e))
        }
    }
}

#[get("/items/{id}")]
async fn get_item_by_id_handler(
    item_id: web::Path<i32>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error getting database connection");

    match get_item_by_id(&mut conn, item_id.into_inner()) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => {
            eprintln!("Failed to get item: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to get item: {}", e))
        }
    }
}

#[delete("/items/{id}")]
async fn delete_item_handler(
    item_id: web::Path<i32>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error getting database connection");

    match delete_item(&mut conn, item_id.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Item deleted"),
        Err(e) => {
            eprintln!("Failed to delete item: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to delete item: {}", e))
        }
    }
}

#[put("/items/{id}")]
async fn update_item_handler(
    item_id: web::Path<i32>,
    item_data: web::Json<UpdateItem>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().expect("Error getting database connection");

    match update_item(&mut conn, item_id.into_inner(), &item_data) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => {
            eprintln!("Failed to update item: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to update item: {}", e))
        }
    }
}

use crate::models::items::models::{CreateItem, Item, UpdateItem};
use crate::models::items::routes;
use actix_web::{test, web, App};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

async fn database_setup() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok(); // Load .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// Helper function to extract item ID from response
async fn extract_item_id(
    resp: actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
) -> i32 {
    let body = test::read_body(resp).await;
    let item: Item = serde_json::from_slice(&body).unwrap();
    item.id
}

async fn init_test_app() -> (
    impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Error = impl std::fmt::Debug,
    >,
    Pool<ConnectionManager<PgConnection>>,
) {
    let pool = database_setup().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::item_routes),
    )
    .await;
    (app, pool)
}

#[actix_web::test]
async fn test_create_item() {
    let (app, _) = init_test_app().await;

    let req = test::TestRequest::post()
        .uri("/items")
        .set_json(CreateItem {
            name: "Test Item".to_string(),
            description: Some("Test Description".to_string()),
            expected_arrival_date: Some(chrono::Utc::now().naive_utc()),
            item_received: Some(false),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let item: Item = serde_json::from_slice(&body).unwrap();
    assert_eq!(item.name, "Test Item");
    assert_eq!(item.description, Some("Test Description".to_string()));
}

#[actix_web::test]
async fn test_get_items() {
    let (app, _) = init_test_app().await;

    // Create an item first
    let create_req = test::TestRequest::post()
        .uri("/items")
        .set_json(CreateItem {
            name: "Test Item".to_string(),
            description: Some("Test Description".to_string()),
            expected_arrival_date: Some(chrono::Utc::now().naive_utc()),
            item_received: Some(false),
        })
        .to_request();
    let _create_resp = test::call_service(&app, create_req).await;

    let req = test::TestRequest::get().uri("/items").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let items: Vec<Item> = serde_json::from_slice(&body).unwrap();
    assert!(!items.is_empty());
}

#[actix_web::test]
async fn test_get_item_by_id() {
    let (app, _) = init_test_app().await;

    // Create an item first
    let create_req = test::TestRequest::post()
        .uri("/items")
        .set_json(CreateItem {
            name: "Test Item".to_string(),
            description: Some("Test Description".to_string()),
            expected_arrival_date: Some(chrono::Utc::now().naive_utc()),
            item_received: Some(false),
        })
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert!(create_resp.status().is_success());

    let item_id = extract_item_id(create_resp).await;
    let req = test::TestRequest::get()
        .uri(&format!("/items/{}", item_id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_delete_item() {
    let (app, _) = init_test_app().await;

    // Create an item first
    let create_req = test::TestRequest::post()
        .uri("/items")
        .set_json(CreateItem {
            name: "Test Item".to_string(),
            description: Some("Test Description".to_string()),
            expected_arrival_date: Some(chrono::Utc::now().naive_utc()),
            item_received: Some(false),
        })
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert!(create_resp.status().is_success());

    let item_id = extract_item_id(create_resp).await;
    let req = test::TestRequest::delete()
        .uri(&format!("/items/{}", item_id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_update_item() {
    let (app, _) = init_test_app().await;

    // Create an item first
    let create_req = test::TestRequest::post()
        .uri("/items")
        .set_json(CreateItem {
            name: "Test Item".to_string(),
            description: Some("Test Description".to_string()),
            expected_arrival_date: Some(chrono::Utc::now().naive_utc()),
            item_received: Some(false),
        })
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert!(create_resp.status().is_success());

    let item_id = extract_item_id(create_resp).await;
    let req = test::TestRequest::put()
        .uri(&format!("/items/{}", item_id))
        .set_json(UpdateItem {
            name: Some("Updated Name".to_string()),
            description: Some("Updated Description".to_string()),
            expected_arrival_date: Some(chrono::Utc::now().naive_utc()),
            item_received: Some(true),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

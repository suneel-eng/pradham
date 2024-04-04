use actix_web::{get, web, HttpResponse};
use sea_orm::EntityTrait;
use std::fs;

use crate::models;
use entity::served_requests_count;

#[get("/")]
pub async fn home_page(data: web::Data<models::AppState>) -> HttpResponse {

    let mut contents = fs::read_to_string("./pradham/templates/index.html").unwrap();

    let rows = served_requests_count::Entity::find().all(&data.db_connection).await;

    if let Err(error) = rows {
       return HttpResponse::InternalServerError().body(format!("500 Internal server error: {}", error.to_string()));
    }

    let mut served_requests_count: u32 = 0;

    for row in rows.unwrap() {
        served_requests_count += row.requests_count;
    }

    contents = contents.replace("{served_requests_count}", served_requests_count.to_string().as_str());

    HttpResponse::Ok().body(contents)
}
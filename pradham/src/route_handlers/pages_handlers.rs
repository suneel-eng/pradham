use actix_web::{get, web, HttpResponse};
use std::fs;

use crate::models;

#[get("/")]
pub async fn home_page(_data: web::Data<models::AppState>) -> HttpResponse {

    let mut contents = fs::read_to_string("./pradham/templates/index.html").unwrap();

    contents = contents.replace("{served_requests_count}", "100");

    HttpResponse::Ok().body(contents)
}